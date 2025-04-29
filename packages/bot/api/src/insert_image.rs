use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize)]
pub struct ImageInsertRequest {
    pub id: u32,
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
