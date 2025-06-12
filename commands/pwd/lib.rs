use std::env;

use shell_common::{CommandResult, ShellCommand};

pub struct PwdCommand;

impl ShellCommand for PwdCommand {
	fn name() -> &'static str {
		"pwd"
	}
	fn execute(_args: &[&str]) -> CommandResult {
		let cwd = env::current_dir().map_err(|e| format!("pwd: {e}"))?;
		println!("{}", cwd.display());
		Ok(())
	}
}
