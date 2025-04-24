use oc_bots_sdk_canister::{HttpRequest, HttpResponse};
use crate::{
    storage::asset::AssetStorage, 
    types::asset::AssetPath
};

pub async fn get(
    request: HttpRequest
) -> HttpResponse {
    let Ok(asset_id) = request.path.trim_start_matches("/assets/").parse::<AssetPath>();

    match AssetStorage::load(asset_id) {
        Some(asset) => {
            HttpResponse::new(200, asset.data, &asset.mime_type)
        },
        None => {
            HttpResponse::not_found()
        }
    }
}
