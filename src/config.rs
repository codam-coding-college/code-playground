// -----------------------------------------------------------------------------
// Codam Coding College, Amsterdam @ 2023.
// See README in the root project for more information.
// -----------------------------------------------------------------------------

use std::{io::Read, net::Ipv4Addr};
use serde::{ Deserialize, Serialize };
use anyhow::{ Context, Error, Result };
use std::{ str::FromStr, path::Path, fs::File };

//===========================================================================//

/// The overall configuration of the playground.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
	/// The networking configuration.
	pub network: NetworkConfig,
	/// The code execution configuration.
	pub executor: ExecutorConfig,
}

impl Config {
	/// Creates a new config from a given path.
	pub fn from_file<FilePath: AsRef<Path>>(file: FilePath) -> Result<Config> {
		let mut buffer = String::new();

		File::open(file)
			.with_context(|| "Unable to open the configuration file")?
			.read_to_string(&mut buffer)
			.with_context(|| "Couldn't read the file")?;

		return Config::from_str(&buffer);
	}
}

impl FromStr for Config {
	type Err = Error;

	/// Load a `Config` from some string.
	fn from_str(src: &str) -> Result<Self> {
		toml::from_str(src).with_context(|| "Invalid configuration file")
	}
}

//===========================================================================//

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct NetworkConfig {
	/// The ip to run the server on.
	pub ip: Ipv4Addr,
	/// The port to listen on.
	pub port: u16,
	/// Timeout in seconds for the requests.
	pub timeout: u16,
	/// Origins to allow incoming requests from.
	pub origins: Vec<String>
}

impl Default for NetworkConfig {
	fn default() -> NetworkConfig {
		NetworkConfig {
			ip: Ipv4Addr::new(127, 0, 0, 1),
			port: 4242,
			timeout: 10,
			origins: vec![".codam.nl".to_string()]
		}
	}
}

//===========================================================================//

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ExecutorConfig {
	pub timeout: u64,
	pub languages: Vec<CodeLanguage>
}

impl Default for ExecutorConfig {
	fn default() -> ExecutorConfig {
		ExecutorConfig {
			timeout: 5000,
			languages: vec![]
		}
	}
}

//===========================================================================//

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct CodeLanguage {
	/// The language name, e.g: `c`, `cpp`, `rust`, this will be used to match with what is requested.
	pub name: String,
	/// The compile command, e.g: `gcc {sourceFile} -o {targetFile}`
	pub compile: Option<String>,
	/// The execute command, e.g: `{targetFile}` or if compile is optional `python3 {targetFile}`
	pub execute: String,
	/// The file extension of the language.
	pub extension: String
}

impl Default for CodeLanguage {
	fn default() -> CodeLanguage {
		CodeLanguage {
			compile: Some("gcc {flags} {sourceFile} -o {targetFile}".to_string()),
			execute: "{targetFile}".to_string(),
			name: "c".to_string(),
			extension: "c".to_string()
		}
	}
}