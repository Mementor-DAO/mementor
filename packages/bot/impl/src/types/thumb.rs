use std::borrow::Cow;

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

pub const THUMB_WIDTH: usize = 256;
pub const THUMB_HEIGHT: usize = 256;
pub const THUMB_FORMAT: image::ImageFormat = image::ImageFormat::Jpeg;
pub const THUMB_MAX_SIZE: u32 = 131072;
pub const THUMB_FONT_SIZE: f32 = 32.0; // px

#[derive(CandidType, Deserialize)]
pub struct Thumb {
    pub data: Vec<u8>
}

impl Storable for Thumb {
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
        max_size: THUMB_MAX_SIZE,
        is_fixed_size: false,
    };
}
