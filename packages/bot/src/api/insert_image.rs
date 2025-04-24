use candid::CandidType;
use serde::{Deserialize, Serialize};
use crate::services::meme::MemeTplId;

#[derive(CandidType, Serialize, Deserialize)]
pub struct ImageInsertRequest {
    pub id: MemeTplId,
    pub mime_type: String,
    pub data: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum ImageInsertResponse {
    Success(usize),
    ImageLoadingFailed,
    ImageGenerationFailed,
    ThumbGenerationFailed,
    NotAuthorized,
    ImageSizeTooBig,
    ThumbSizeTooBig,
}
