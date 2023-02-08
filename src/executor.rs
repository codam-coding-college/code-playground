// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

use anyhow::{Ok, Error, Result};
use wait_timeout::ChildExt;
use std::process::{Command, Stdio, Child, Output};
use uuid::Uuid;
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
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

/// Believe me this was a pain to write.
/// 
/// Basically waits and returns the output of a child process and waits for it
/// or returns err if the child timesout.
fn wait_with_output_timeout(mut child: Child, duration: &Duration) -> Result<Output> {
	// Yeet stdin, we don't need it.
	drop(child.stdin.take());

	// Populate either stdout or stderr.
	let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
	match (child.stdout.take(), child.stderr.take()) {

		(None, None) => {}
		(Some(mut out), None) => {
			out.read_to_end(&mut stdout)?;
		}
		(None, Some(mut err)) => {
			err.read_to_end(&mut stderr)?;
		}
		(Some(mut out), Some(mut err)) => {
			out.read_to_end(&mut stdout)?;
			err.read_to_end(&mut stderr)?;
		}
	}

	// Now wait for the child, error on timeout.
	match child.wait_timeout(*duration)? {
		Some(status) => Ok(Output { status, stdout, stderr }),
		None => Err(Error::msg("Timeout!"))
	}
}

/// Executes a full command with a timeout, if the Result is none the
/// command timed out, in case of error the result is used.
/// 
/// If it all went well, stdout is returned.
/// 
/// E.G: gcc bruh.c -o a.out
fn execute_with_timeout(command: String, duration: &Duration) -> Result<String> {

	// Split arguments from program executable.
	// TODO: It might be a bit naive to assume that the its always just at the first space.
	let mut split_command = command.split_whitespace();
	let executable = match split_command.next() {
		Some(x) => x,
		None => return Err(Error::msg("Failed to split command, check config!"))
	};

	let child = Command::new(executable)
		.args(split_command)
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.map_err(Error::from)?;

	let output = wait_with_output_timeout(child, duration)?;
	let stderr = String::from_utf8(output.stderr).map_err(Error::from)?;

	// Shit went south
	if !output.status.success() || !stderr.is_empty() {
		let exit_code = match output.status.code() {
			Some(x) => x,
			None => 1, // Technically a deadly signal was sent.
		};

		return Err(Error::msg(format!(
			"Code: {exit_code}\nOutput: {stderr}\n",
		)));
	}

	// Yeet STDOUT back out!
	Ok(String::from_utf8(output.stdout).map_err(Error::from)?)
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

		let path = Path::new("/tmp/code");
		std::fs::create_dir_all(path)?;

		let exec_path = path.join(id.to_string());
		let srcs_path = path.join(format!("{}.{}", id, self.content.extension));
		let timeout = Duration::from_secs(self.content.timeout);

		// Write the source code into the file
		let mut source_file = File::create(srcs_path.clone())?;
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
		return Ok(execute_with_timeout(execute, &timeout)?);
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
