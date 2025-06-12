use shell_cd::CdCommand;
use shell_common::ShellCommand;
use shell_ls::LsCommand;
use shell_pwd::PwdCommand;

pub fn is_builtin(cmd: &str) -> bool {
	matches!(cmd, "cd" | "exit" | "pwd" | "ls" | "help" | "reload")
}

pub fn exec_builtin(cmd: &str, args: &[&str]) -> Result<(), String> {
	match cmd {
		"cd" => {
			CdCommand::execute(args)?;
			Ok(())
		}
		"exit" => {
			std::process::exit(0);
		}
		"pwd" => {
			PwdCommand::execute(args)?;
			Ok(())
		}
		"ls" => {
			LsCommand::execute(args)?;
			Ok(())
		}
		"help" => {
			help()?;
			Ok(())
		}
		"reload" => {
			todo!();
		}
		_ => Ok(()),
	}
}

fn help() -> Result<(), String> {
	println!(" cd <dir>  - Change directory");
	println!(" pwd       - Print working directory");
	println!(" ls        - List files in the current directory");
	println!(" exit      - Exit shell-rs");
	println!(" reload    - Reserved keyword config reloading");

	Ok(())
}

// TODO: add config reloading
#[allow(dead_code)]
fn reload() {
	todo!();
}
