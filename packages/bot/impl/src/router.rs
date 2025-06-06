use ic_http_certification::{HttpRequest, HttpResponse};
use oc_bots_sdk_canister::{HttpMethod::*, HttpRouter};
use std::sync::LazyLock;

mod definition;
mod commands;
mod blobs;
mod assets;

static ROUTER: LazyLock<HttpRouter> = LazyLock::new(init_router);

fn init_router(
) -> HttpRouter {
    HttpRouter::default()
        .route("/execute_command", POST, commands::execute)
        //.route("/webhook/*", POST, webhooks::execute)
        //.route("/metrics", GET, metrics::get)
        .route("/blobs/*", GET, blobs::get)
        .route("/assets/*", GET, assets::get)
        .fallback(definition::get)
}

pub async fn handle(
    request: HttpRequest, 
    query: bool
) -> HttpResponse {
    ROUTER.handle(request, query).await
}