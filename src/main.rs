// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

use std::{ net::SocketAddr };
use axum::{response::Html, routing::post, Router};

pub mod config;

#[tokio::main]
async fn main() {
	let config = config::Config::from_file("./Config.toml").unwrap();
	let app = Router::new().route("/playground", post(execute_code));
	let addr = SocketAddr::from((config.network.ip, config.network.port));

	println!("Running on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn execute_code() -> Html<&'static str> {
	Html("<h1>Hello, World!</h1>")
}
