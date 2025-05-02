use std::{collections::HashSet, io::Cursor, sync::LazyLock, u32};
use async_trait::async_trait;
use candid::Principal;
use clap::Parser;
use ic_ledger_types::{
    AccountIdentifier, DEFAULT_FEE, DEFAULT_SUBACCOUNT
};
use oc_bots_sdk::{
    api::{
        command::{
            CommandHandler, EphemeralMessageBuilder, SuccessResult
        }, 
        definition::*
    }, 
    oc_api::{
        actions::{
            chat_details, chat_events::{
                self, EventsSelectionCriteria, EventsWindowArgs
            }, send_message, ActionArgsBuilder
        }, 
        client::Client
    }, 
    types::{
        ActionContext, BlobReference, BotCommandContext, BotCommandScope, 
        Chat, ImageContent, MessageContentInitial
    }
};
use oc_bots_sdk_canister::{env, CanisterRuntime};
use crate::{
    services::{
        meme::{self, MemeService},
        nft::{self, NftService, TOKENS_PER_PAGE, CHECK_POH_AFTER}, 
        wallet::wallet::WalletService
    }, 
    state, 
    storage::{
        blob::BlobStorage, 
        nft::NftStorage, 
        user::UserStorage
    }, 
    types::{
        blob::Blob, 
        cli::{self, Cli, Commands}, 
        image::{IMAGE_FORMAT, IMAGE_HEIGHT, IMAGE_WIDTH}, 
        meme::MemeId, 
        nft::{Nft, NftId}, 
        user::{UserMeme, UserMint, UserPost, UserTransaction}
    }, 
    utils::{
        image::{create_thumbnail, rgba8_to_rgb8}, 
        oc::{get_chat_user_profile, get_user_pub_profile}, 
        out_font::OutlinedFont
    }
};

static DEFINITION: LazyLock<BotCommandDefinition> = LazyLock::new(MemeCli::definition);

const IMG_FORMAT: image::ImageFormat = image::ImageFormat::Jpeg;
const LOG_ITEMS_PER_PAGE: usize = 8;

pub struct MemeCli;

#[async_trait]
impl CommandHandler<CanisterRuntime> for MemeCli {
    fn definition(
        &self
    ) -> &BotCommandDefinition {
        &DEFINITION
    }

    async fn execute(
        &self,
        client: Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {

        let args = shell_words::split(
            &format!("/meme {}", client.context().command.arg::<String>("args"))
        ).unwrap();

        let user_id = Principal::from_text(
            client.context().command.initiator.to_string()
        ).unwrap();

        let BotCommandScope::Chat(chat_scope) = &client.context().scope else {
            return Err("This command can only be used in a chat".to_string());
        };

        let chat = chat_scope.chat;

        let res = match Cli::try_parse_from(args) {
            Ok(cli) => {
                match cli.command {
                    Commands::Search { query, page } => {
                        Self::search_meme(query, page.max(1) - 1, &client)
                    },
                    Commands::Gen { id, captions } => {
                        Self::gen_meme(id, captions, user_id, &client)
                    },
                    Commands::Suggest { id, mood, topic } => {
                        Self::suggest_meme(
                            id, 
                            mood.unwrap_or("funny".to_string()), 
                            topic.unwrap_or("crypto".to_string()), 
                            user_id, 
                            &client
                        ).await
                    },
                    Commands::Post { id } => {
                        Self::post_meme(id, user_id, &client)
                    },
                    Commands::Wallet (command) => {
                        match command {
                            cli::Wallet::Balance => {
                                Self::wallet_balance(user_id, &client)
                                    .await
                            },
                            cli::Wallet::Address => {
                                Self::wallet_address(user_id, &client)
                                    .await
                            },
                            cli::Wallet::Withdraw { to, amount } => {
                                Self::wallet_withdraw(user_id, to, amount, &client)
                                    .await
                            },
                            cli::Wallet::Logs { page } => {
                                Self::wallet_logs(
                                    user_id,
                                    page.max(1) - 1,
                                    &client
                                )
                            },
                        }
                    },
                    Commands::Nft (command) => {
                        match command {
                            cli::Nft::Mint { id } => {
                                Self::nft_mint(id, user_id, &chat, &client)
                                    .await
                            },
                            cli::Nft::Balance { owner } => {
                                Self::nft_balance_of(
                                    owner.map(|t| Principal::from_text(t).unwrap())
                                        .or(Some(user_id)).unwrap(), 
                                    &client
                                ).await
                            },
                            cli::Nft::Tokens { owner, page } => {
                                Self::nft_tokens_of(
                                    owner.map(|t| Principal::from_text(t).unwrap())
                                        .or(Some(user_id)).unwrap(), 
                                    page.max(1) - 1,
                                    &client
                                ).await
                            },
                            cli::Nft::Status { } => { 
                                Self::nft_status(
                                    &client
                                ).await
                            },
                            cli::Nft::Transfer { id, to } => { 
                                if let Ok(to) = Principal::from_text(to.clone()) {
                                    Self::nft_transfer_from(
                                        user_id,
                                        id,
                                        to,
                                        &client
                                    ).await
                                }
                                else {
                                    Err(format!("Invalid principal for 'to': {}", to))
                                }
                            },
                            cli::Nft::Logs { page } => {
                                Self::nft_logs(
                                    user_id,
                                    page.max(1) - 1,
                                    &client
                                )
                            },
                        }
                    },
                }
            },
            Err(err) => {
                Err(match err.kind() {
                    clap::error::ErrorKind::DisplayVersion => {
                        err.to_string()
                    },
                    _ => {
                        ansi_to_html::Converter::new()
                            .convert(
                                &err.render()
                                    .ansi()
                                    .to_string()
                            ).unwrap()
                    }
                })
            },
        };

        match res {
            Ok(success_res) => {
                Ok(success_res)
            },
            Err(text) => {
                Ok(EphemeralMessageBuilder::new(
                    MessageContentInitial::from_text(text),
                    client.context().message_id().unwrap(),
                )
                    .with_block_level_markdown(true)
                    .build()
                    .into()
                )
            }
        }
    }
}

impl MemeCli {
    fn search_meme(
        query: String,
        page: usize, 
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        // find the templates that match the query used
        let (tpls, num_pages) = meme::read(|s| 
            s.search(query.as_str(), page)
        );

        if tpls.len() > 0 {
            // create a preview with the memes found
            let preview = OutlinedFont::roboto(|font| {
                match MemeService::gen_preview(&tpls, font) {
                    Ok(preview) => {
                        Ok(rgba8_to_rgb8(&preview))
                    },
                    Err(err) => {
                        Err(err)
                    },
                }
            })?;

            let mut jpeg: Vec<u8> = Vec::new();
            preview.write_to(&mut Cursor::new(&mut jpeg), IMG_FORMAT)
                .map_err(|e| e.to_string())?;

            let thumbnail_data = create_thumbnail(
                &jpeg, 
                preview.width() / 5,
                preview.height() / 5,
                IMG_FORMAT
            )?;

            let blob_id = BlobStorage::save(Blob {
                data: jpeg,
                mime_type: IMG_FORMAT.to_mime_type().to_string()
            }, true);

            // return a message to user only
            Ok(EphemeralMessageBuilder::new(
                MessageContentInitial::Image(ImageContent {
                    mime_type: IMG_FORMAT.to_mime_type().to_string(),
                    width: preview.width(),
                    height: preview.height(),
                    caption: Some(format!("Page {}/{}", page.min(num_pages-1)+1, num_pages)),
                    blob_reference: Some(BlobReference {
                        canister_id: env::canister_id(),
                        blob_id,
                    }),
                    thumbnail_data,
                }),
                client.context().message_id().unwrap()
            ).build().into())
        }
        else {
            Err("No memes found :/. Try again!".to_string())
        }
    }

    fn gen_meme(
        tpl_id: u32,
        captions: Vec<String>,
        user_id: Principal,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        if let Some(tpl) = meme::read(|s| 
            s.load(&tpl_id).cloned()
        ) {
            // gen the image
            let img = OutlinedFont::roboto(|font| {
                rgba8_to_rgb8(&MemeService::gen_image(
                    &tpl, 
                    &captions.iter().map(|t| t.to_uppercase()).collect(), 
                    &font
                ).unwrap())
            });

            let mut jpeg: Vec<u8> = Vec::new();
            img.write_to(&mut Cursor::new(&mut jpeg), IMG_FORMAT)
                .map_err(|e| e.to_string())?;

            let thumbnail_data = create_thumbnail(
                &jpeg, 
                img.width() / 5,
                img.height() / 5,
                IMG_FORMAT
            )?;

            // add meme generated to user DB
            let mut user = UserStorage::load(&user_id);

            let temp_blob_id = BlobStorage::save(Blob {
                data: jpeg,
                mime_type: IMG_FORMAT.to_mime_type().to_string()
            }, true);

            let meme_id = meme::mutate(|s| {
                s.calc_id(
                    &tpl, 
                    &captions
                )
            });

            user.memes.list.insert(
                meme_id.clone(), 
                UserMeme::new(
                    meme_id.clone(),
                    temp_blob_id,
                )
            );
            user.memes.last = Some(meme_id.clone());
            UserStorage::save(user_id, user);

            // return a message to user only
            Ok(EphemeralMessageBuilder::new(
                MessageContentInitial::Image(ImageContent {
                    mime_type: IMG_FORMAT.to_mime_type().to_string(),
                    width: img.width(),
                    height: img.height(),
                    caption: Some(format!("meme id: **{}**", meme_id)),
                    blob_reference: Some(BlobReference {
                        canister_id: env::canister_id(),
                        blob_id: temp_blob_id,
                    }),
                    thumbnail_data,
                }),
                client.context().message_id().unwrap()
            ).build().into())
        }
        else {
            Err("Unknown meme :/".to_string())
        }
    }

    async fn suggest_meme(
        tpl_id: u32,
        mood: String,
        topic: String,
        user_id: Principal,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        if let Some(tpl) = meme::read(|s| 
            s.load(&tpl_id).cloned()
        )  {
            let captions = MemeService::gen_captions(&tpl, mood, topic)
                .await?;

            Self::gen_meme(tpl_id, captions, user_id, client)
            }
        else {
            Err("Unknown meme :/".to_string())
        }
    }

    fn post_meme(
        meme_id: Option<MemeId>,
        user_id: Principal,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let mut user = UserStorage::load(&user_id);

        if user.memes.list.len() > 0 {
            // find meme generated
            let meme = if let Some(meme_id) = meme_id {
                if !meme_id.is_empty() {
                    user.memes.list.get(&meme_id).cloned()
                }
                else {
                    None
                }
            }
            else {
                None
            }.or_else(|| {
                Some(user.memes.last.clone()
                    .map(|id| user.memes.list.get(&id).unwrap())
                    .unwrap()
                    .clone()
                )
            });
            
            if let Some(meme) = meme {
                // generate image
                let jpeg = BlobStorage::load(meme.tmp_blob_id).unwrap();

                let thumbnail_data = create_thumbnail(
                    &jpeg.data, 
                    IMAGE_WIDTH / 5,
                    IMAGE_HEIGHT / 5,
                    IMAGE_FORMAT
                )?;

                let blob_id = BlobStorage::save(jpeg, false);

                let content = ImageContent {
                    mime_type: IMAGE_FORMAT.to_mime_type().to_string(),
                    width: IMAGE_WIDTH,
                    height: IMAGE_HEIGHT,
                    caption: None,
                    blob_reference: Some(BlobReference {
                        canister_id: env::canister_id(),
                        blob_id,
                    }),
                    thumbnail_data,
                };

                // post to group/channel 
                Ok(SuccessResult { 
                    message: client
                        .send_message(MessageContentInitial::Image(content))
                        .execute_then_return_message(move |args, response| match response {
                            Ok(send_message::Response::Success(msg)) => {
                                user.posts.list.insert(
                                    meme.meme_id.clone(),
                                    UserPost{ 
                                        blob_id,
                                        meme_id: meme.meme_id.clone(),
                                        message_id: msg.message_id, 
                                        message_index: msg.message_index,
                                    }
                                );
                                user.posts.last = Some(meme.meme_id);
                                UserStorage::save(user_id, user);
                            }
                            error => {
                                ic_cdk::println!("send_message: {args:?}, {error:?}");
                            }
                        })
                })
            }
            else {
                Err("No meme found!".to_string())
            }
        }
        else {
            Err("No meme found. Use /meme_gen first!".to_string())
        }
    }

    async fn wallet_balance(
        user_id: Principal,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {

        let icps = WalletService::balance_of(
            user_id
        ).await?;

        let content = format!(
            "Balance:  \nICP: {:.8}  \n", 
            icps as f32 / 100000000.0
        );
        
        Ok(
            EphemeralMessageBuilder::new(
                MessageContentInitial::Text(content.into()), 
                client.context().message_id().unwrap()
            ).with_block_level_markdown(true)
            .build()
            .into()
        )
    }

    async fn wallet_address(
        user_id: Principal,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let icp_acc_id = WalletService::address_of(
            user_id
        );

        let content = format!(
            "Address:  \nICP: {}  \n", 
            icp_acc_id
        );
        
        Ok(
            EphemeralMessageBuilder::new(
                MessageContentInitial::Text(content.into()), 
                client.context().message_id().unwrap()
            ).with_block_level_markdown(true)
            .build()
            .into()
        )
    }

    async fn wallet_withdraw(
        user_id: Principal,
        to: Option<String>,
        amount: f32,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let (block_num, account_id) = WalletService::transfer_hex(
            user_id,
            to,
            (amount * 100000000.0) as u64
        ).await?;

        let content = format!(
            "Withdrawn of **{}** ICP to account id **{}** completed! At block index: **{}**", 
            amount, account_id, block_num
        );
        
        Ok(
            EphemeralMessageBuilder::new(
                MessageContentInitial::Text(content.into()), 
                client.context().message_id().unwrap()
            ).with_block_level_markdown(true)
            .build()
            .into()
        )
    }

    fn wallet_logs(
        user_id: Principal,
        page_num: usize,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let user = UserStorage::load(&user_id);

        let logs = user.txs.iter()
            .skip(page_num * LOG_ITEMS_PER_PAGE)
            .take(LOG_ITEMS_PER_PAGE)
            .cloned()
            .map(|tx| match tx {
                UserTransaction::IcpWithdraw { amount, to, block_num, timestamp } => 
                    format!(
                        "Withdraw: amount({} ICP) to account_id({}) with block_num({}) at timestamp({})", 
                        amount as f32 / 100000000.0, to, block_num, timestamp
                    ),
                _ => "".to_string()
            })
            .collect::<Vec<_>>()
            .join("  \n");

        let num_pages = (user.txs.len() + LOG_ITEMS_PER_PAGE-1) / LOG_ITEMS_PER_PAGE;
        let page_num = (1+page_num).min(num_pages);

        Ok(
            EphemeralMessageBuilder::new(
                MessageContentInitial::Text(
                    format!("{}  \n  \nPage {}/{}",
                        if logs.len() > 0 {
                            logs
                        } 
                        else {
                            "No transactions found".to_string()
                        },
                        page_num,
                        num_pages
                    ).into()
                ), 
                client.context().message_id().unwrap()
            ).with_block_level_markdown(true)
                .build()
                .into()
        )
    }

    async fn nft_mint(
        meme_id: Option<MemeId>,
        user_id: Principal,
        chat: &Chat,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let mut user = UserStorage::load(&user_id);

        let nft_service = nft::read(|s| {
            s.clone()
        });
        let meme_nft_canister_id = nft_service.col.canister_id.clone();

        if user.posts.list.len() == 0 {
            return Err("Meme not found. Use /meme_post first!".to_string());
        }

        // find user post
        let post = if let Some(meme_id) = meme_id {
            if !meme_id.is_empty() {
                user.posts.list.get(&meme_id).cloned()
            }
            else {
                None
            }
        }
        else {
            None
        }.or_else(|| {
            Some(user.posts.last.clone()
                .map(|id| user.posts.list.get(&id).unwrap())
                .unwrap()
                .clone()
            )
        });

        if post.is_none()  {
            return Err("Post not found!".to_string());
        }
        
        let post = post.unwrap();

        // check if NFT was already minted
        if let Some(nft) = NftStorage::load_by_meme_id(&post.meme_id) {
            return Err(
                format!(
                    "Meme was already minted! Token id: **{}**", 
                    nft.token_id
                )
            );
        }

        let total_supply = nft_service.total_supply();

        if total_supply > CHECK_POH_AFTER {
            // check if user has proof of uniqueness
            let has_poh = match get_chat_user_profile(chat, &user_id).await {
                Some(profile) => {
                    !profile.user_type.is_bot() &&
                        profile.unique_person_proof.is_some()
                },
                None => {
                    false
                }
            };

            if !has_poh {
                return Err(format!("You haven't proven to be a unique individual yet. Please do that first on Open Chat"));
            }
        }

        // check if group/channel has enough members
        let min_chat_members = nft_service.min_chat_members();
        let num_chat_members = if min_chat_members > 0 { 
            Self::get_num_chat_members(client).await.unwrap_or(u32::MAX)
        }
        else {
            u32::MAX
        };

        if num_chat_members < min_chat_members {
            return Err(
                format!("The group/channel where the meme was posted has only {} members. Expected at least {}", 
                    num_chat_members, min_chat_members)
            );
        }

        // check if post has enough reactions
        let min_reactions = nft_service.calc_min_reactions();
        let num_reactions = if min_reactions > 0 {
            Self::count_unique_reactions_from_poh_users(
                &post, chat, client
            ).await
        }
        else {
            0
        };

        if num_reactions < min_reactions {
            return Err(
                format!("Post has only **{}** reactions from different users created {} days ago or earlier who have proven to be unique individuals. Expected at least **{}** reactions. Try harder 😎", 
                    num_reactions, nft_service.min_user_creation_interval_in_days(), min_reactions)
            );
        }

        // check user ICP balance
        let cost = nft_service.calc_minting_cost();
        let balance = WalletService::balance_of(user_id).await?;
        if balance < cost {
            let acc_id = WalletService::address_of(user_id);
            return Err(
                format!(
                    "Your Mementor wallet balance of **{:.8}** ICP is too low to cover the current minting cost of **{:.8}** ICP  \nPlease transfer enough ICP to this address: **{}**", 
                    (balance as f32) / 100000000.0,
                    (cost as f32) / 100000000.0,
                    acc_id
                )
            );
        }

        // get blob generated by the user post
        let jpeg = BlobStorage::load(post.blob_id).unwrap();

        let thumbnail_data = create_thumbnail(
            &jpeg.data, 
            IMAGE_WIDTH / 5,
            IMAGE_HEIGHT / 5,
            IMAGE_FORMAT
        )?;

        // debit user ICP wallet
        if let Err(err) = WalletService::transfer(
            user_id.into(), 
            AccountIdentifier::new(&ic_cdk::id(), &DEFAULT_SUBACCOUNT), 
            cost
        ).await {
            let err = format!(
                "Failed to debit the minting cost: {}.", 
                err
            );
            ic_cdk::println!("error: {}", err);
            return Err(err);
        };

        // mint the NFT with transfer approval to our bot 
        // NOTE: this is needed while OC doesn't support ICRC-7 NFTs on its wallet. by now users can't transfer their NFTs directly
        let (token_id, meta) = match 
                nft_service.mint_with_approval(
                    user_id.into(), 
                    &post.meme_id,
                    post.blob_id,
                    num_reactions
                ).await {
            Ok(meta) => {
                meta
            },
            Err(err) => {
                ic_cdk::println!("error: NFT minting failed: {}", err);

                // return the value transferred to user
                if let Err(err) = WalletService::transfer(
                    None, 
                    AccountIdentifier::new(&ic_cdk::id(), &user_id.into()), 
                    cost + DEFAULT_FEE.e8s()
                ).await {
                    ic_cdk::println!(
                        "error: Could not return minting cost {} to user {}: {}", 
                        cost, 
                        user_id.to_text(), 
                        err
                    );
                };

                return Err(err);
            },
        };

        // store the nft in our DB
        NftStorage::save(token_id, &post.meme_id, Nft::new(
            token_id,
            post.blob_id,
            Some(meta),
            (ic_cdk::api::time() / 1_000_000_000) as _
        ));

        user.mints.list.insert(
            post.meme_id.clone(),
            UserMint{
                canister_id: meme_nft_canister_id, 
                token_id,
                timestamp: ic_cdk::api::time() / 1_000_000,
            }
        );
        user.mints.last = Some(post.meme_id.clone());
        UserStorage::save(user_id, user);

        // transfer the dev team's cut
        let team_account = state::read(|s| s.administrator().clone());
        let team_fee_p = nft::read(|n| n.config.team_fee_p);
        if let Err(err) = WalletService::transfer(
            None, 
            AccountIdentifier::new(&team_account, &DEFAULT_SUBACCOUNT), 
            (cost * team_fee_p) / 1_00000000
        ).await {
            ic_cdk::println!("error: Failed to transfer the dev team fee: {}", err);
        };

        // return a message to user only
        Ok(EphemeralMessageBuilder::new(
            MessageContentInitial::Image(
                ImageContent {
                    mime_type: IMAGE_FORMAT.to_mime_type().to_string(),
                    width: IMAGE_WIDTH,
                    height: IMAGE_HEIGHT,
                    caption: Some(format!("NFT **{}** has just been minted! 🎉🎉🎉", token_id)),
                    blob_reference: Some(BlobReference {
                        canister_id: env::canister_id(),
                        blob_id: post.blob_id,
                    }),
                    thumbnail_data,
                }
            ),
            client.context().message_id().unwrap()
        ).build().into())
    }

    async fn count_unique_reactions_from_poh_users(
        post: &UserPost,
        chat: &Chat,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> u32 {
        let initiator = client.context().command.initiator.clone();

        match client
                .chat_events(EventsSelectionCriteria::Window(EventsWindowArgs {
                    mid_point: post.message_index,
                    max_messages: 1,
                    max_events: 1,
                }))
                .execute_async()
                .await {
            Ok(chat_events::Response::Success(res)) => {
                let users = res.events.first().and_then(|event| match &event.event {
                    oc_bots_sdk::types::ChatEvent::Message(msg) => {
                        if msg.message_index == post.message_index {
                            let initiator = initiator.to_string();
                            let mut users = HashSet::new();
                            msg.reactions.iter().for_each(|reaction| 
                                reaction.1.iter().for_each(|user_id| {
                                    let user_id = (*user_id).to_string();
                                    if user_id != initiator {
                                        users.insert(user_id);
                                    }
                                })
                            );
                            Some(users)
                        }
                        else {
                            None
                        }
                    },
                    _ => None
                });

                match users {
                    Some(users) => {
                        let now = ic_cdk::api::time() / 1_000_000;
                        let min_user_creation_interval = nft::read(|s| s.min_user_creation_interval());
                        let mut count = 0;
                        for user in &users {
                            let user_id = Principal::from_text(user).unwrap();
                            match get_user_pub_profile(&user_id).await {
                                Some(prof) => {
                                    // only consider the reaction if user creation time is old enough
                                    if (now - prof.created) >= min_user_creation_interval {
                                        // and if user has proof of humanity (and is not a bot)
                                        match get_chat_user_profile(chat, &user_id).await {
                                            Some(g_user) => {
                                                if !g_user.user_type.is_bot() && 
                                                    g_user.unique_person_proof.is_some() {
                                                    count += 1;
                                                }
                                            },
                                            None => {
                                            }
                                        }
                                    }
                                },
                                None => {
                                }
                            }
                        }

                        count
                    },
                    None => {
                        0
                    }
                }
            }
            _ => {
                0
            }
        }
    }

    async fn get_num_chat_members(
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Option<u32> {
        match client.chat_details().execute_async().await {
            Ok(chat_details::Response::Success(details)) => {
                Some(details.member_count)
            },
            Err(err) => {
                ic_cdk::println!("error: getting chat details: {}", err.1);
                None
            },
            _ => {
                ic_cdk::println!("error: getting chat details");
                None
            }
        }
    }

    async fn nft_balance_of(
        user_id: Principal,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let nft_service = nft::read(|s| s.clone());

        let balance = nft_service.balance_of(
            user_id.into()
        ).await?;

        let content = format!("Balance: {}", balance);
        
        Ok(
            EphemeralMessageBuilder::new(
                MessageContentInitial::Text(content.into()), 
                client.context().message_id().unwrap()
            ).with_block_level_markdown(true)
            .build()
            .into()
        )
    }

    async fn nft_tokens_of(
        user_id: Principal,
        page_num: usize,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let nft_service = nft::read(|s| s.clone());

        let balance = nft_service.balance_of(
            user_id.into()
        ).await?;

        if balance == 0 {
            return Ok(
                EphemeralMessageBuilder::new(
                    MessageContentInitial::Text("No tokens found. Go and mint a NFT before it's too late! 😋".to_string().into()), 
                    client.context().message_id().unwrap()
                ).with_block_level_markdown(true)
                .build()
                .into()
            );
        }

        let num_pages = (balance as usize + TOKENS_PER_PAGE-1) / TOKENS_PER_PAGE;
        let page_num = page_num.min(num_pages - 1);

        let tokens = nft_service.tokens_of(
            user_id.into(), 
            page_num
        ).await?;
        
        // gen a preview with the user tokens
        let preview = OutlinedFont::roboto(|font| {
            rgba8_to_rgb8(&NftService::gen_preview(&tokens, font).unwrap())
        });
        
        let mut jpeg: Vec<u8> = Vec::new();
        preview.write_to(&mut Cursor::new(&mut jpeg), IMG_FORMAT)
            .map_err(|e| e.to_string())?;

        let thumbnail_data = create_thumbnail(
            &jpeg, 
            preview.width() / 5,
            preview.height() / 5,
            IMG_FORMAT
        )?;

        let blob_id = BlobStorage::save(Blob {
            data: jpeg,
            mime_type: IMG_FORMAT.to_mime_type().to_string()
        }, true);
        
        // return a message to user only
        Ok(EphemeralMessageBuilder::new(
            MessageContentInitial::Image(ImageContent {
                mime_type: IMG_FORMAT.to_mime_type().to_string(),
                width: preview.width(),
                height: preview.height(),
                caption: Some(format!(
                    "Page {}/{}", 1+page_num, num_pages)
                ),
                blob_reference: Some(BlobReference {
                    canister_id: env::canister_id(),
                    blob_id,
                }),
                thumbnail_data,
            }),
            client.context().message_id().unwrap()
        ).build().into())
    }

    async fn nft_transfer_from(
        from: Principal,
        token_id: NftId,
        to: Principal,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let nft_service = nft::read(|s| s.clone());

        let Some(nft) = NftStorage::load(&token_id) else {
            return Err(format!("NFT {} not found", token_id));
        };

        nft_service.transfer_from(
            from, 
            nft.token_id,
            to.into()
        ).await?;

        Ok(
            EphemeralMessageBuilder::new(
                MessageContentInitial::Text(
                    format!("Token {} transferred to {}! 🎉", 
                        token_id, to.to_text()
                    ).to_string().into()
                ), 
                client.context().message_id().unwrap()
            ).with_block_level_markdown(true)
                .build()
                .into()
        )
    }

    fn nft_logs(
        user_id: Principal,
        page_num: usize,
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let user = UserStorage::load(&user_id);

        let logs = user.txs.iter()
            .skip(page_num * LOG_ITEMS_PER_PAGE)
            .take(LOG_ITEMS_PER_PAGE)
            .cloned()
            .map(|tx| match tx {
                UserTransaction::NftTransfer { token_id, to, tx_id, timestamp } => 
                    format!(
                        "Transfer: token_id({}) to principal({}) with tx_id({}) at timestamp({})", 
                        token_id, to.owner, tx_id, timestamp
                    ),
                _ => "".to_string()
            })
            .collect::<Vec<_>>()
            .join("  \n");

        let num_pages = (user.txs.len() + LOG_ITEMS_PER_PAGE-1) / LOG_ITEMS_PER_PAGE;
        let page_num = (1+page_num).min(num_pages);

        Ok(
            EphemeralMessageBuilder::new(
                MessageContentInitial::Text(
                    format!("{}  \n  \nPage {}/{}",
                        if logs.len() > 0 {
                            logs
                        } 
                        else {
                            "No transactions found".to_string()
                        },
                        page_num,
                        num_pages
                    ).into()
                ), 
                client.context().message_id().unwrap()
            ).with_block_level_markdown(true)
                .build()
                .into()
        )
    }

    async fn nft_status(
        client: &Client<CanisterRuntime, BotCommandContext>
    ) -> Result<SuccessResult, String> {
        let nft_service = nft::read(|s| {
            s.clone()
        });

        let minting_cost = nft_service.calc_minting_cost();
        let min_reactions = nft_service.calc_min_reactions();
        //let min_chat_members = nft_service.min_chat_members();
        let max_supply = nft_service.col.max_supply;
        let total_supply = NftStorage::size();

        let text = format!(
            "**MEME NFT Status**  \n- minting cost: **{:.8} ICP**  \n- min reactions: {}  \n- supply: {}/{}  \n",
            minting_cost as f32 / 1_00000000.0,
            min_reactions,
            total_supply,
            max_supply
        );

        Ok(
            EphemeralMessageBuilder::new(
                MessageContentInitial::Text(text.into()), 
                client.context().message_id().unwrap()
            ).with_block_level_markdown(true)
                .build()
                .into()
        )
    }

    fn definition(
    ) -> BotCommandDefinition {
        BotCommandDefinition {
            name: "meme".to_string(),
            description: Some("Mementor's command interface. Type -h for help".to_string()),
            placeholder: Some("Please wait...".to_string()),
            params: vec![
                BotCommandParam {
                    name: "args".to_string(),
                    description: Some("Arguments separated by white-spaces (leave empty for help)".to_string()),
                    placeholder: Some("Enter the arguments".to_string()), 
                    required: false,
                    param_type: BotCommandParamType::StringParam(StringParam{
                        choices: vec![],
                        min_length: 0,
                        max_length: 1024,
                        multi_line: true,
                    }),
                },
            ],
            permissions: BotPermissions::default().with_message(&HashSet::from([
                MessagePermission::Text,
                MessagePermission::Image
            ])).with_chat(&HashSet::from([
                ChatPermission::ReadMessages,
                ChatPermission::ReadChatDetails,
            ])),
            default_role: None,
            direct_messages: Some(true),
        }
    }
}

