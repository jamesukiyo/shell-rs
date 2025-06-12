use std::io::stdin;

use crate::{exec, prompt};

pub struct Shell {
	config: shell_config::Config,
}

impl Shell {
	pub fn new() -> Self {
		Shell {
			config: shell_config::Config::load().unwrap_or_default(),
		}
	}

	pub fn run(&mut self) {
		loop {
			prompt::display_prompt(&self.config)
				.expect("Failed to display prompt");

			let input = match self.read_input() {
				Ok(input) => input,
				Err(e) => {
					eprintln!("Error reading input: {e}");
					continue;
				}
			};

			if input.trim().is_empty() {
				continue;
			}

			if let Err(e) = self.exec_line(&input) {
				eprintln!("Error: {e}");
			}
		}
	}

	fn read_input(&self) -> Result<String, std::io::Error> {
		let mut input = String::new();
		stdin().read_line(&mut input)?;
		Ok(input)
	}

	fn exec_line(&mut self, input: &str) -> Result<(), String> {
		exec::exec_command(input.trim())?;
		Ok(())
	}
}
