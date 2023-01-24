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
	pub execute: ExecuteConfig,
}

impl FromStr for Config {
	type Err = Error;

	/// Load a `Config` from some string.
	fn from_str(src: &str) -> Result<Self> {
		toml::from_str(src).with_context(|| "Invalid configuration file")
	}
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

//===========================================================================//

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct NetworkConfig {
	pub ip: Ipv4Addr,
	pub port: u16,
}

impl Default for NetworkConfig {
	fn default() -> NetworkConfig {
		NetworkConfig {
			ip: Ipv4Addr::new(127, 0, 0, 1),
			port: 4242
		}
	}
}

//===========================================================================//

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ExecuteConfig {
	pub timeout: u32,
}

impl Default for ExecuteConfig {
	fn default() -> ExecuteConfig {
		ExecuteConfig {
			timeout: 5000
		}
	}
}