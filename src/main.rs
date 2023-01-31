// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

mod api;
mod config;
mod executor;

use log::info;
use std::sync::Arc;
use config::Config;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use axum::{http::{self, HeaderValue, Method}, routing::post, Router};

// State
//===========================================================================//

#[derive(Clone)]
pub struct AppState {
	config: Config,
}

//===========================================================================//

/// Create the cors layer
fn get_cors_layer(state: Arc<AppState>) -> CorsLayer {
	let origins = state.config.network.origins
		.iter()
		.map(|s| s.parse::<HeaderValue>().unwrap())
		.collect::<Vec<HeaderValue>>();

	return CorsLayer::new()
		// .allow_origin(origins)
		// .allow_methods([Method::POST])
		.allow_headers([http::header::CONTENT_TYPE]); // For JSON
}

//===========================================================================//

#[tokio::main]
async fn main() {
	info!("Starting playground...");

	// Create state
	let config = Config::from_file("./Config.toml").unwrap();
	let state = Arc::new(AppState { config });

	// Construct routes and address
	let addr = SocketAddr::from((state.config.network.ip, state.config.network.port));
	let app = Router::new()
		.route("/playground", post(api::playground::handle))
		.route_layer(get_cors_layer(state.clone()))
		.with_state(state)
		.into_make_service();

	// Run the server
	info!("Running on: {}", addr);
	axum::Server::bind(&addr).serve(app).await.unwrap();
}
