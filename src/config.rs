use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub emoji: String,
	pub display_name: String,
	pub separator: String,
	pub char: String,
	pub insert_blank_line: bool,
	pub show_dir: bool,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			emoji: "🦀".to_string(),
			display_name: "shell-rs".to_string(),
			separator: "@".to_string(),
			char: ">".to_string(),
			insert_blank_line: true,
			show_dir: true,
		}
	}
}

impl Config {
	pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
		let config_path = Self::config_path()?;

		if !config_path.exists() {
			// Create a default config file
			let default_config = Config::default();
			default_config.save()?;
			return Ok(default_config);
		}

		let content = fs::read_to_string(&config_path)?;
		let config: Config = toml::from_str(&content)?;
		Ok(config)
	}

	pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
		let config_path = Self::config_path()?;

		// Create config directory if it doesnt exist
		if let Some(parent) = config_path.parent() {
			fs::create_dir_all(parent)?;
		}

		// Convert config to toml string
		let content = toml::to_string_pretty(self)?;
		fs::write(&config_path, content)?;
		Ok(())
	}

	fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
		let home = std::env::var("HOME")
			.or_else(|_| std::env::var("USERPROFILE"))
			.map_err(|_| "Could not find home directory")?;

		Ok(PathBuf::from(home)
			.join(".config")
			.join("shell-rs")
			.join("config.toml"))
	}
}
