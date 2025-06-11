use std::env;

pub fn is_builtin(cmd: &str) -> bool {
	matches!(cmd, "cd" | "exit" | "pwd" | "ls" | "help" | "reload")
}

pub fn exec_builtin(cmd: &str, args: &[&str]) -> Result<(), String> {
	match cmd {
		"cd" => {
			cd(args)?;
			Ok(())
		}
		"exit" => {
			std::process::exit(0);
		}
		"pwd" => {
			pwd()?;
			Ok(())
		}
		"ls" => {
			ls()?;
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

fn cd(args: &[&str]) -> Result<(), String> {
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

fn pwd() -> Result<(), String> {
	let cwd = env::current_dir().map_err(|e| format!("pwd: {e}"))?;
	println!("{}", cwd.display());
	Ok(())
}

// TODO: improve output - prefixed with `./`
// https://doc.rust-lang.org/std/fs/fn.read_dir.html
fn ls() -> Result<(), String> {
	// Collect all items in the current directory with read_dir iterator
	let mut entries: Vec<_> = std::fs::read_dir(".")
		.map_err(|e| format!("ls: {e}"))?
		.collect::<Result<Vec<_>, _>>()
		.map_err(|e| format!("ls: {e}"))?;

	// Has to be sorted because order is not guranteed
	entries.sort_by_key(|entry| entry.path());

	for entry in entries {
		println!("{}", entry.path().display());
	}

	Ok(())
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
