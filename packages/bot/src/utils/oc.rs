use candid::Principal;
use ic_cdk::api::call::call_raw;
use oc_bots_sdk::types::{ChannelId, Chat};
use serde::Serialize;
use user_canister::public_profile::PublicProfile;
use super::msgpack::serialize_to_vec;

#[derive(Serialize)]
struct Empty {}

pub async fn is_user(
    prin: &Principal
) -> bool {
    match call_raw(
        prin.clone(),
        "public_profile_msgpack",
        serialize_to_vec(Empty{}).unwrap(),
        0
    ).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn get_user_pub_profile(
    prin: &Principal
) -> Option<PublicProfile> {
    use user_canister::public_profile::Response;

    match call_raw(
        prin.clone(),
        "public_profile_msgpack",
        serialize_to_vec(Empty{}).unwrap(),
        0
    ).await {
        Ok(buf) => {
            let res: Response = rmp_serde::from_slice(&buf).unwrap();
            match res {
                Response::Success(public_profile) => {
                    Some(public_profile)
                },
            }
        },
        Err(_) => None,
    }
}

pub async fn get_num_chat_members(
    chat: &Chat
) -> Option<u32> {
    // NOTE: call doesn't work because a bot isn't considered a group/channel member :/
    if let Some(channel_id) = chat.channel_id() {
        get_num_channel_members(chat.canister_id(), channel_id).await
    }
    else {
        get_num_group_members(chat.canister_id()).await
    }
}

async fn get_num_group_members(
    canister_id: Principal
) -> Option<u32> {
    use group_canister::public_summary::{Args, Response};

    match call_raw(
        canister_id,
        "public_summary_msgpack",
        serialize_to_vec(Args{ 
            invite_code: None 
        }).unwrap(),
        0
    ).await {
        Ok(buf) => {
            let res: Response = rmp_serde::from_slice(&buf).unwrap();
            match res {
                Response::Success(res) => {
                    Some(res.summary.participant_count)
                },
                Response::Error(err) => {
                    ic_cdk::println!("error: calling {canister_id}.public_summary_msgpack: {:?}", err);
                    None
                }
            }
        },
        Err(err) => {
            ic_cdk::println!("error: calling {canister_id}.public_summary_msgpack: {}", err.1);
            None
        },
    }
}

async fn get_num_channel_members(
    canister_id: Principal,
    channel_id: ChannelId
) -> Option<u32> {
    use community_canister::channel_summary::{Args, Response};

    match call_raw(
        canister_id,
        "channel_summary_msgpack",
        serialize_to_vec(Args{ 
            channel_id: channel_id.into(), 
            invite_code: None 
        }).unwrap(),
        0
    ).await {
        Ok(buf) => {
            let res: Response = rmp_serde::from_slice(&buf).unwrap();
            match res {
                Response::Success(summary) => {
                    Some(summary.member_count)
                },
                Response::Error(err) => {
                    ic_cdk::println!("error: calling {canister_id}.channel_summary_msgpack: {:?}", err);
                    None
                }
            }
        },
        Err(err) => {
            ic_cdk::println!("error: calling {canister_id}.channel_summary_msgpack: {}", err.1);
            None
        },
    }
}