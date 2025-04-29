use crate::{
    resources::LOGO, 
    services::nft, 
    types::{
        asset::{Asset, AssetPath}, 
        blob::BlobId
    }
};

pub struct AssetStorage;

impl AssetStorage {
    pub fn load(
        path: AssetPath
    ) -> Option<Asset> {
        if path == "nft_logo.png" {
            Some(Asset{
                mime_type: "image/png".to_string(),
                data: LOGO.to_vec(),
            })
        }
        else if let Ok(Ok(nft_id)) = path.trim_start_matches("nfts/").parse::<String>()
            .map(|s| s.trim_end_matches(".jpg").parse::<BlobId>()) {
            nft::read(|s| s.get_token_image(nft_id))
        }
        else {
            None
        }
    }
}