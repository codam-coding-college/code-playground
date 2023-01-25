// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

use anyhow::{ Result };
use uuid::Uuid;
use tempfile::tempdir;
use std::fs::File;
use std::io::{Write};
use std::process::{Stdio, Command};
use std::time::Duration;

//===========================================================================//

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleParams {
	/// The code to execute.
	pub code: String,
	/// The flags passed onto the compiler / interpreter.
	pub flags: String,
	/// The file extension used for the source file.
	pub extension: String,
	/// Code execution timeout.
	pub timeout: u64,
}

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
	content: ModuleParams
}

impl CompiledExecutor {
	pub fn new(compile_cmd: String, execute_cmd: String, content: ModuleParams) -> Self {
		return CompiledExecutor { compile_cmd, execute_cmd, content };
	}
}

impl Execute for CompiledExecutor {
	fn execute(&self) -> Result<String> {
		// Create temporary directory with the source file in it
		let id = Uuid::new_v4();
		let dir = tempdir()?;
		let file_path = dir.path().join(format!("{}.{}", id, self.content.extension));
		let timeout = Duration::from_secs(self.content.timeout);

		// Write the source code into the file
		let mut source_file = File::create(file_path)?;
		source_file.write_all(self.content.code.as_bytes())?;

		// Compile process
		let compile = self.compile_cmd
			.replace("{sourceFile}", "todo")
			.replace("{targetFile}", "todo");
		let mut _compile_child = Command::new(compile)
			.stderr(Stdio::piped())
			.spawn()
			.expect("Failed to compile!");
		
		// _compile_child.wait_timeout()
		// TODO: Check stderr, if it is not None, Err.

		// Execution process
		let execute = self.execute_cmd
			.replace("{targetFile}", "todo");
		let mut _execute_child = Command::new(execute)
			.stdin(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn()
			.expect("Failed to execute!");

		// TODO: Check stderr, if it is not None, Err.
		// TODO: Pass stdout to Ok.

		dir.close()?;
		return Ok(String::from("todo"));
	}
}

// Interpreted
//===========================================================================//

/// Executor for interpreted languages such as:
/// `Python`, `Ruby`, `Javascript`, ...
struct InterpretedExecutor {
	execute_cmd: String,
	content: ModuleParams
}

impl InterpretedExecutor {
	pub fn new(execute_cmd: String, content: ModuleParams) -> Self {
		return InterpretedExecutor { execute_cmd, content };
	}
}

impl Execute for InterpretedExecutor {
	fn execute(&self) -> Result<String> {
		todo!("Not implemented yet...")
	}
}
