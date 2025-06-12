use shell_common::{CommandResult, ShellCommand};

pub struct LsCommand;

impl ShellCommand for LsCommand {
	fn name() -> &'static str {
		"ls"
	}
	// https://doc.rust-lang.org/std/fs/fn.read_dir.html
	fn execute(args: &[&str]) -> CommandResult {
		// Collect all items in the current directory with read_dir iterator
		let mut entries: Vec<_> =
			std::fs::read_dir(args.first().unwrap_or(&"."))
				.map_err(|e| format!("ls: {e}"))?
				.collect::<Result<Vec<_>, _>>()
				.map_err(|e| format!("ls: {e}"))?;

		// Has to be sorted because order is not guranteed
		entries.sort_by_key(|entry| entry.path());

		for entry in entries {
			let path = entry.path();
			let display_path = path.strip_prefix("./").unwrap_or(&path);
			println!("{}", display_path.display());
		}

		Ok(())
	}
}
