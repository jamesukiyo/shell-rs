use std::env;
use std::io::{Write, stdout};

use crate::config::Config;

// TODO: maybe there's a cleaner way to do this?
pub fn display_prompt(config: &Config) -> Result<(), std::io::Error> {
	let cwd = env::current_dir()?;
	if config.insert_blank_line {
		println!(" ");
	}

	if config.show_dir {
		println!(
			"{} {} {} {}",
			config.emoji,
			config.display_name,
			config.separator,
			cwd.display()
		);
	} else {
		println!("{} {}", config.emoji, config.display_name);
	}
	print!("{} ", config.char);
	stdout().flush()?;
	Ok(())
}
