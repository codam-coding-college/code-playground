// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// ---------0----------------------1----------------------------------------

mod api;
mod config;
mod executor;

use log::info;
use config::Config;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::{
	Router,
	routing::post,
	http::{Request, StatusCode},
	response::Response,
	middleware::{Next, self}, extract::State,
};

// State
//===========================================================================//

#[derive(Clone)]
pub struct AppState {
	config: Config,
}

// Main
//===========================================================================//

/// Filter requests by content-type.
///
/// https://docs.rs/axum/latest/axum/middleware/fn.from_fn.html
async fn filter<B>(
	State(state): State<Arc<AppState>>,
	request: Request<B>,
	next: Next<B>,
) -> Result<Response, StatusCode> {
	let headers = request.headers();

	if headers["content-type"] != "application/json" {
		return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
	}

	let response = next.run(request).await;
	Ok(response)
}

#[tokio::main]
async fn main() {
	info!("Starting playground...");

	let config = Config::from_file("./Config.toml").unwrap();
	let state = Arc::new(AppState { config });

	// Construct routes and address
	let addr = SocketAddr::from((state.config.network.ip, state.config.network.port));
	let app = Router::new()
		.route("/playground", post(api::playground::handle))
		.route_layer(middleware::from_fn_with_state(state.clone(), filter))
		.with_state(state);

	// Run the server
	info!("Running on: {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}
