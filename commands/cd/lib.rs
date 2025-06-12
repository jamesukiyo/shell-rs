use std::env;

use shell_common::{CommandResult, ShellCommand};

pub struct CdCommand;

impl ShellCommand for CdCommand {
	fn name() -> &'static str {
		"cd"
	}
	fn execute(args: &[&str]) -> CommandResult {
		let target_dir = if args.is_empty() {
			// No arg so go to ~ like other shells
			env::var("HOME").map_err(|_| "cd: HOME not set".to_string())?
		} else {
			let dir = args[0];
			if dir == "~" {
				env::var("HOME").map_err(|_| "cd: HOME not set".to_string())?
			} else {
				dir.to_string()
			}
		};

		env::set_current_dir(&target_dir).map_err(|e| format!("cd: {e}"))?;

		Ok(())
	}
}
