use oc_bots_sdk_canister::{HttpRequest, HttpResponse};
use crate::{
    storage::blob::BlobStorage, 
    types::blob::BlobId
};

pub async fn get(
    request: HttpRequest
) -> HttpResponse {
    let Ok(blob_id) = request.path.trim_start_matches("/blobs/").parse::<BlobId>() else {
        return HttpResponse::not_found();
    };

    match BlobStorage::load(blob_id) {
        Some(blob) => {
            HttpResponse::new(200, blob.data, &blob.mime_type)
        },
        None => {
            HttpResponse::not_found()
        }
    }
}
