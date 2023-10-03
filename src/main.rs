pub(crate) mod backends;
mod errors;
mod state;
pub(crate) mod utils;

use axum::{
    debug_handler,
    extract::{Query, State},
    routing::get,
    Json, Router, Server,
};
use backends::crtsh::CrtshClientImpl;
use state::AppState;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::errors::AppError;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let crtsh_client = CrtshClientImpl::default();
    let state = Arc::new(AppState {
        crtsh_client: Box::new(crtsh_client),
    });

    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/api/domain", get(get_domains))
        .with_state(state);

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match std::env::var(port_key) {
        Ok(val) => val.parse().expect("could not parse port"),
        Err(_) => 3000,
    };
    Server::bind(&([0, 0, 0, 0], port).into())
        .serve(app.into_make_service())
        .await
        .unwrap();

    // curl command for testing domain
    // curl -i http://localhost:3000/api/v1/domains/example.com
}

#[debug_handler]
async fn get_domains(
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<HashSet<String>>, AppError> {
    let domain = match params.get("q") {
        Some(domain) => domain,
        None => return Err(AppError::MissingDomainParam),
    };

    Ok(Json(state.crtsh_client.get_domains(&domain).await?))
}
