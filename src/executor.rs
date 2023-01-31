// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

use anyhow::{ Result, Error, bail };
use uuid::Uuid;
use tempfile::tempdir;
use wait_timeout::ChildExt;
use std::fs::File;
use std::io::{Write, Read};
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

// Functions
//===========================================================================//

/// Executes a command with a timeout, if the Result is none the
/// command timed out, in case of error the result is used.
/// 
/// If it all went well, stdout is returned.
fn execute_with_timeout(command: String, timeout: &Duration) -> Result<String> {
	let mut child = Command::new(command)
	.stdout(Stdio::piped())
	.stderr(Stdio::piped())
	.spawn()?;

	let mut stderr = String::new();
	if let Some(ref mut err) = child.stderr {
		err.read_to_string(&mut stderr)?;
		return Err(Error::msg(stderr));
	}
	
	let mut stdout = String::new();
	if let Some(ref mut out) = child.stdout {
		out.read_to_string(&mut stdout)?;
	}

	// Wait for it to finish.
	let status = child.wait_timeout(*timeout)?;
	if let Some(exit_code) = status {
		if !exit_code.success() {
			return Err(Error::msg("Process did not execute sucessfully"));
		}
		return Ok(stdout);
	} else {
		return Err(Error::msg("Timeout!"));
	}
}

// Compiled
//===========================================================================//

/// Executor for interpreted languages such as:
/// `C`, `C++`, ...
pub struct CompiledExecutor {
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
		let exec_path = dir.path().join(id.to_string());
		let srcs_path = dir.path().join(format!("{}.{}", id, self.content.extension));
		let timeout = Duration::from_secs(self.content.timeout);

		// Write the source code into the file
		let mut source_file = File::create(srcs_path.clone())?;
		println!("{:?}", srcs_path.to_str());
		source_file.write_all(self.content.code.as_bytes())?;


		// Compile process
		// TODO: Do not panic here!
		let compile = self.compile_cmd
			.replace("{sourceFile}", srcs_path.to_str().unwrap())
			.replace("{targetFile}", exec_path.to_str().unwrap());
		if let Err(err) = execute_with_timeout(compile, &timeout) {
			return Err(err);
		}

		// Compile process
		// TODO: Do not panic here!
		let execute = self.execute_cmd
			.replace("{targetFile}", exec_path.to_str().unwrap());
		let execute_out = match execute_with_timeout(execute, &timeout) {
			Ok(x) => x,
			Err(e) => return Err(e)
		};

		dir.close()?;
		return Ok(execute_out);
	}
}

// Interpreted
//===========================================================================//

/// Executor for interpreted languages such as:
/// `Python`, `Ruby`, `Javascript`, ...
pub struct InterpretedExecutor {
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
