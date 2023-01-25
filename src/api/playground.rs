// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

use std::sync::Arc;

use axum::{response::{Json, IntoResponse}, extract::State, http::StatusCode};
use log::info;
use serde::{Deserialize, Serialize};

use crate::AppState;

// Request and Response
//===========================================================================//

/// The request payload.
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaygroundRequest {
	/// The code to execute.
	code: String,
	/// The language shortcut (e.g. `py` for Python).
	language: String,
	/// Optional flags for the compiler / interpreter.
	flags: Option<String>,
}

/// The response payload.
#[derive(Deserialize, Serialize, Debug)]
pub struct PlaygroundResponse {
	/// The successfull output of the execution.
	output: Option<String>,
	/// The error output of the execution.
	error: Option<String>,
}

// Route handlers
//===========================================================================//

/// The route handler.
pub async fn handle(
	State(state): State<Arc<AppState>>,
	Json(payload): Json<PlaygroundRequest>
) -> impl IntoResponse {
	println!("{:?}", payload);

	return (StatusCode::CREATED, Json(PlaygroundResponse {
		error: None,
		output: None,
	}));
}
