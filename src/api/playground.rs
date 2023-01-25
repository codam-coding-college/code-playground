// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

use std::sync::Arc;

use axum::{
	extract::State,
	http::StatusCode,
	response::{IntoResponse, Json},
};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{AppState, executor::ModuleParams};

// Request and Response
//===========================================================================//

/// The request payload.
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaygroundRequest {
	/// The code to execute.
	pub code: String,
	/// The language shortcut (e.g. `py` for Python).
	pub language: String,
	/// Optional flags for the compiler / interpreter.
	pub flags: Option<String>,
}

/// The response payload.
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaygroundResponse {
	/// The successfull output of the execution.
	pub output: Option<String>,
	/// The error output of the execution.
	pub error: Option<String>,
}

// Route handlers
//===========================================================================//

/// The route handler.
pub async fn handle(
	State(state): State<Arc<AppState>>,
	Json(payload): Json<PlaygroundRequest>,
) -> impl IntoResponse {
	println!("Received request: {}", payload.language);

	let code_iter = state.config.executor.languages
		.iter()
		.find(|&x| x.name == payload.language);

	let code_lang = match code_iter {
		None => return (
			StatusCode::UNSUPPORTED_MEDIA_TYPE, Json(PlaygroundResponse { 
				error: Some(String::from("Unsupported playground language")),
				output: None
			})
		),
		Some(lang) => lang
	};

	let module = ModuleParams {
		extension: code_lang.extension.clone(),
		timeout: state.config.executor.timeout,
		code: payload.code,
		flags: match payload.flags {
			None => String::from(""),
			Some(x) => x
		},
	};

	return (
		StatusCode::OK,
		Json(PlaygroundResponse {
			error: None,
			output: None,
		}),
	);
}
