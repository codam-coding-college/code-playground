// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

use log::info;
use std::net::SocketAddr;
use axum::{response::Json, routing::post, Router};

pub mod config;
pub mod executor;

//===========================================================================//

#[tokio::main]
async fn main() {
	info!("Starting playground...");

	// Read configuration file
	let config = config::Config::from_file("./Config.toml").unwrap();

	// Construct routes and address
	let app = Router::new().route("/playground", post(execute_code));
	let addr = SocketAddr::from((config.network.ip, config.network.port));
	info!("Running on: {}", addr);

	// Run the server
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn execute_code() -> Json<&'static str> {
	Json("{}")
}
