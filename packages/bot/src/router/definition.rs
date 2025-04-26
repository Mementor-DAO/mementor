use super::commands;
use oc_bots_sdk::api::definition::*;
use oc_bots_sdk_canister::{HttpRequest, HttpResponse};

pub async fn get(
    _request: HttpRequest
) -> HttpResponse {
    HttpResponse::json(
        200,
        &BotDefinition {
            description: "Mementor lets you create hilarious memes, mint them as exclusive MEME NFTs, and earn MEME coins in return! 🎨🚀🔥"
                .to_string(),
            commands: commands::definitions(),
            autonomous_config: None,
        },
    )
}