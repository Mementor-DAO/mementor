use candid::Principal;
use ic_cdk::api::call::call_raw;
use local_user_index_canister::GlobalUser;
use oc_bots_sdk::types::Chat;
use serde::{Deserialize, Serialize};
use user_canister::public_profile::PublicProfile;
use super::msgpack::serialize_to_vec;

#[derive(Serialize)]
struct Empty {}

#[derive(Deserialize)]
pub enum LocalUserIndexCanisterIdResponse {
    Success(Principal),
}

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

async fn get_local_user_index_canister_id(
    chat: &Chat
) -> Option<Principal> {

    match call_raw(
        chat.canister_id(),
        "local_user_index_msgpack",
        serialize_to_vec(Empty{}).unwrap(),
        0
    ).await {
        Ok(buf) => {
            let res: LocalUserIndexCanisterIdResponse = rmp_serde::from_slice(&buf).unwrap();
            match res {
                LocalUserIndexCanisterIdResponse::Success(canister_id) => {
                    Some(canister_id)
                }
            }
        },
        Err(err) => {
            ic_cdk::println!("error: calling {}.local_user_index_msgpack: {:?}", chat.canister_id(), err);
            None
        }
    }
}

pub async fn get_chat_user_profile(
    chat: &Chat,
    prin: &Principal
) -> Option<GlobalUser> {
    use local_user_index_canister::c2c_lookup_user::{Args, Response};
    if let Some(canister_id) = get_local_user_index_canister_id(chat).await {
        match call_raw(
            canister_id,
            "c2c_lookup_user_msgpack",
            serialize_to_vec(Args{
                user_id_or_principal: *prin
            }).unwrap(),
            0
        ).await {
            Ok(buf) => {
                let res: Response = rmp_serde::from_slice(&buf).unwrap();
                match res {
                    Response::Success(global_user) => {
                        Some(global_user)
                    },
                    Response::UserNotFound => {
                        None
                    },
                }
            },
            Err(err) => {
                ic_cdk::println!("error: calling {}.c2c_lookup_user_msgpack: {:?}", canister_id, err);
                None
            },
        }
    }
    else {
        None
    }
}