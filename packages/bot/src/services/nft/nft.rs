use candid::Principal;
use icrc7_types::{
    icrc37_types::{TransferFromArg, TransferFromResult}, 
    icrc7_types::{Icrc7TokenMetadata, MintArg, MintResult}
};
use icrc_ledger_types::{
    icrc::generic_value::Value, 
    icrc1::account::Account
};
use image::{Rgba, RgbaImage};
use tiny_skia::Color;
use crate::{
    storage::{
        blob, 
        event::EventStorage, 
        image::ImageStorage, 
        nft::NftStorage, 
        user::UserStorage
    }, 
    types::{
        asset::Asset, 
        blob::BlobId, 
        event::Event, 
        image::IMAGE_FORMAT, 
        meme::MemeId, 
        nft::{Nft, NftId}, 
        nft_col::{NftCollection, NftCollectionConfig}, 
        user::UserTransaction
    }, 
    utils::{
        canvas::{Canvas, Point}, 
        nat::nat_to_u128, 
        oc, 
        out_font::OutlinedFont
    }
};

pub const TOKENS_PER_ROW: usize = 4;
pub const TOKENS_PER_PAGE: usize = TOKENS_PER_ROW * 1;

const PADDING: usize = 8;
const PREVIEW_IMG_WIDTH: usize = 256;
const PREVIEW_IMG_HEIGHT: usize = 256;

#[derive(Clone, Default)]
pub struct NftService {
    pub config: NftCollectionConfig,
    pub col: NftCollection,
}

impl NftService {
    pub fn update(
        &mut self,
        config: NftCollectionConfig,
        col: NftCollection
    ) {
        self.config = config;
        self.col = col;
    }

    pub fn calc_minting_cost(
        &self
    ) -> u64 {
        // y = min + floor(tanh(minted) * (max - min))
        let total_supply = NftStorage::size() as u32;
        self.config.min_minting_cost + (
            (total_supply as f32 / self.col.max_supply as f32).tanh() * 
            (self.config.max_minting_cost - self.config.min_minting_cost) as f32
        ).floor() as u64
    }

    pub fn calc_min_reactions(
        &self
    ) -> u32 {
        // y = min + floor(tanh(minted) * (max - min))
        let total_supply = NftStorage::size() as u32;
        self.config.min_num_reactions + (
            (total_supply as f32 / self.col.max_supply as f32).tanh() * 
            (self.config.max_num_reactions - self.config.min_num_reactions) as f32
        ).floor() as u32
    }

    pub fn min_chat_members(
        &self
    ) -> u32 {
        self.config.min_chat_members
    }

    pub fn min_user_creation_interval(
        &self
    ) -> u64 {
        self.config.min_user_creation_interval
    }

    pub fn min_user_creation_interval_in_days(
        &self
    ) -> u64 {
        self.config.min_user_creation_interval / (24 * 60 * 60 * 1_000)
    }

    pub fn get_token_image(
        &self,
        token_id: BlobId
    ) -> Option<Asset> {
        blob::BlobStorage::load(token_id)
            .map(|b| Asset{
                data: b.data, 
                mime_type: b.mime_type 
            }
        )
    }

    pub async fn balance_of(
        &self,
        owner: Account
    ) -> Result<u32, String> {
        Ok(ic_cdk::call::<(Vec<Account>, ), (Vec<u128>, )>(
            self.col.canister_id.clone(),
            "icrc7_balance_of", 
            (vec![owner], )
        ).await.map_err(|e| e.1)?.0[0] as u32)
    }

    pub async fn tokens_of(
        &self,
        owner: Account,
        page_num: usize
    ) -> Result<Vec<Nft>, String> {
        let mut res = vec![];
        
        let mut prev = None;
        while res.len() < (page_num+1) * TOKENS_PER_PAGE {
            let ids = ic_cdk::call::<(Account, Option<u128>, Option<u128>), (Vec<u128>, )>(
                self.col.canister_id.clone(),
                "icrc7_tokens_of", 
                (owner, prev, Some(TOKENS_PER_PAGE as u128))
            ).await.map_err(|e| e.1)?.0;

            if ids.len() == 0 {
                break;
            }
            
            prev = ids.last().cloned();

            let metas = ic_cdk::call::<(Vec<u128>, ), (Vec<Option<Icrc7TokenMetadata>>, )>(
                self.col.canister_id.clone(),
                "icrc7_token_metadata", 
                (ids.clone(), )
            ).await.map_err(|e| e.1)?.0;

            let is_last_page = metas.len() < TOKENS_PER_PAGE;

            res.append(&mut metas.iter()
                .zip(ids)
                .map(|i| Nft::new(
                    i.1,
                    0, 
                    i.0.clone(),
                    0
                ))
                .collect()
            );

            if is_last_page {
                break;
            }
        }

        Ok(res.iter()
            .skip(page_num as usize * TOKENS_PER_PAGE)
            .take(TOKENS_PER_PAGE)
            .cloned()
            .collect::<Vec<_>>()
        )
    }

    pub async fn mint_with_approval(
        &self,
        to: Account,
        meme_id: &MemeId,
        blob_id: BlobId,
        num_reactions: u32
    ) -> Result<(NftId, Icrc7TokenMetadata), String> {
        let url = self.col.url_template.replace("{}", &blob_id.to_string());
        let metadata = Icrc7TokenMetadata::from([
            ("Name".to_string(), Value::Text(format!("#{}", meme_id))),
            ("Logo".to_string(), Value::Text(url.clone())),
            ("BlobId".to_string(), Value::Nat(blob_id.into())),
            ("Reactions".to_string(), Value::Nat64(num_reactions.into())),
        ]);

        let (_tx_id, token_id) = ic_cdk::call::<(MintArg, ), (MintResult, )>(
            self.col.canister_id.clone(),
            "mint_and_grant_transfer_approval", 
            (MintArg {
                from_subaccount: None,
                to,
                token_id: None,
                memo: None,
                meta: metadata.clone()
            }, )
        ).await.map_err(|e| e.1)?.0
            .map_err(|e| format!("{:?}", e))?;

        EventStorage::save(Event::NftMinted { 
            token_id, 
            to, 
            timestamp: (ic_cdk::api::time() / 1_000_000_000) as _
        });
        
        Ok((token_id, metadata))
    }

    pub async fn transfer_from(
        &self,
        from: Principal,
        token_id: u128,
        to: Account
    ) -> Result<u128, String> {

        let is_oc_user = oc::is_user(&to.owner).await;
        let now = ic_cdk::api::time();

        let arg = TransferFromArg {
            spender_subaccount: None,
            from: from.into(),
            to,
            token_id,
            memo: None,
            created_at_time: Some(now)
        };

        let res = if is_oc_user {
            Some(ic_cdk::call::<(TransferFromArg, ), (TransferFromResult, )>(
                self.col.canister_id.clone(),
                "transfer_from_and_grant_transfer_approval",
                (arg, )
            ).await.map_err(|e| e.1)?.0)
        }
        else {
            ic_cdk::call::<(Vec<TransferFromArg>, ), (Vec<Option<TransferFromResult>>, )>(
                self.col.canister_id.clone(),
                "icrc37_transfer_from",
                (vec![arg], )
            ).await.map_err(|e| e.1)?.0[0].clone()
        };

        match res {
            Some(res) => {
                match res {
                    Ok(tx_id) => {
                        let mut user = UserStorage::load(&from);
                        user.txs.push(UserTransaction::NftTransfer { 
                            token_id, 
                            to, 
                            tx_id, 
                            timestamp: (now / 1_000_000_000) as _,
                        });
                        UserStorage::save(from, user);

                        Ok(tx_id)
                    },
                    Err(msg) => Err(format!("{:?}", msg)),
                }
            },
            None => {
                Err("Token not found".to_string())
            },
        }
    }

    pub fn gen_preview(
        tokens: &Vec<Nft>,
        font: &OutlinedFont
    ) -> Result<RgbaImage, String> {
        let mut out = RgbaImage::from_pixel(
            ((PADDING + PREVIEW_IMG_WIDTH) * tokens.len().min(TOKENS_PER_ROW) + PADDING) as _, 
            ((PADDING + PREVIEW_IMG_HEIGHT) * ((tokens.len() + TOKENS_PER_ROW-1) / TOKENS_PER_ROW) + PADDING) as _,
            Rgba([239, 239, 239, 255])
        );

        let mut canvas = Canvas::new(&mut out);

        let text_color = Color::from_rgba8(0xf7, 0x78, 0x00, 0xff);

        let mut x = PADDING;
        let mut y = PADDING;
        for (i, nft) in tokens.iter().enumerate() {
            let blob_id = nat_to_u128(nft.meta.as_ref().unwrap()
                .get("BlobId").unwrap().clone().as_nat().unwrap());

            if let Some(jpeg) = blob::BlobStorage::load(blob_id) {
                let img = image::load_from_memory_with_format(&jpeg.data, IMAGE_FORMAT)
                    .map_err(|e| e.to_string())?
                    .to_rgba8();
                
                let img = ImageStorage::resize(
                    &img, PREVIEW_IMG_WIDTH as _, PREVIEW_IMG_HEIGHT as _
                );

                let dx = PREVIEW_IMG_WIDTH - (img.width() as usize);
                let dy = PREVIEW_IMG_HEIGHT - (img.height() as usize);

                canvas.blit_image_at(&img, (x + dx/2) as _, (y + dy/2) as _)
                    .map_err(|e| e.to_string()).unwrap();

                canvas.draw_text(
                    &nft.token_id.to_string(), 
                    32.0, font, &text_color, 
                    (x + dx/2) as _, (y + dy/2) as _,
                    Some(img.width() as f32), Some(img.height() as f32)
                );

                canvas.draw_rect(
                    PREVIEW_IMG_WIDTH as _, PREVIEW_IMG_HEIGHT as _,
                    &text_color, 
                    &Point::new(x as _, y as _)
                );
            }

            if (i+1) % TOKENS_PER_ROW == 0 {
                x = PADDING;
                y += PREVIEW_IMG_HEIGHT + PADDING;
            }
            else {
                x += PREVIEW_IMG_WIDTH + PADDING;
            }
        }

        Ok(out)
    }
}