use std::borrow::Cow;

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

pub const IMAGE_WIDTH: u32 = 512;
pub const IMAGE_HEIGHT: u32 = 512;
pub const IMAGE_FORMAT: image::ImageFormat = image::ImageFormat::Jpeg;
pub const IMAGE_MAX_SIZE: u32 = 262144;

#[derive(CandidType, Deserialize)]
pub struct Image {
    pub data: Vec<u8>,
}

impl Storable for Image {
    fn to_bytes(
        &self
    ) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(
        bytes: std::borrow::Cow<[u8]>
    ) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: IMAGE_MAX_SIZE,
        is_fixed_size: false,
    };
}