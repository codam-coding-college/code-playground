// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

use anyhow::{ Result };

//===========================================================================//

/// Trait for execution modules.
pub trait Execute {
	/// Returns the result of executing the module or contains any error.
	fn execute(&self) -> Result<String>;
}

// Compiled
//===========================================================================//

/// Executor for interpreted languages such as:
/// `C`, `C++`, ...
struct CompiledExecutor {
	compile_cmd: String,
	execute_cmd: String,
}

impl CompiledExecutor {
	pub fn new(compile_cmd: String, execute_cmd: String) -> Self {
		return CompiledExecutor { compile_cmd, execute_cmd };
	}
}

impl Execute for CompiledExecutor {
	fn execute(&self) -> Result<String> {
		return Ok(String::from("todo"));
	}
}

// Interpreted
//===========================================================================//

/// Executor for interpreted languages such as:
/// `Python`, `Ruby`, `Javascript`, ...
struct InterpretedExecutor {
	execute_cmd: String,
}

impl InterpretedExecutor {
	pub fn new(execute_cmd: String) -> Self {
		return InterpretedExecutor { execute_cmd };
	}
}

impl Execute for InterpretedExecutor {
	fn execute(&self) -> Result<String> {
		Ok(String::from("Hi!"))
	}
}
