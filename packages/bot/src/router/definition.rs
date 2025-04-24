use super::commands;
use oc_bots_sdk::api::definition::*;
use oc_bots_sdk_canister::{HttpRequest, HttpResponse};

pub async fn get(
    _request: HttpRequest
) -> HttpResponse {
    HttpResponse::json(
        200,
        &BotDefinition {
            description: "Mementor lets you easily create hilarious memes and even mint exclusive NFTs! 🎨🚀🔥"
                .to_string(),
            commands: commands::definitions(),
            autonomous_config: None,
        },
    )
}