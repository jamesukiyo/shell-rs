#[warn(clippy::pedantic)] // For learning/experimenting
mod clap_args;

use clap::Parser;
use clap_args::LsArgs;
use shell_common::{CommandResult, ShellCommand};

pub struct LsCommand;

impl ShellCommand for LsCommand {
	fn name() -> &'static str {
		"ls"
	}
	// Parse args and then exec ls
	fn execute(args: &[&str]) -> CommandResult {
		match Self::parse_args(args) {
			Ok(parsed_args) => Self::execute_with_args(parsed_args),
			// TODO: Prevent error when using --help
			Err(e) => {
				eprintln!("{e}");
				Err("Invalid arguments, please try again".to_string())
			}
		}
	}
}

impl LsCommand {
	// This was confusing to me but it works, thanks to a little help from
	// Claude chat too..
	fn parse_args(args: &[&str]) -> Result<LsArgs, String> {
		let args_with_program =
			std::iter::once("ls").chain(args.iter().copied());
		LsArgs::try_parse_from(args_with_program).map_err(|e| e.to_string())
	}

	// I have to credit a few posts online for this one because I almost didn't
	// consider a number of things including the "1024" part somehow and got
	// stuck on the conversions a little. Maybe there is a better way to do this
	// with some crate for simplicity, idk. But it works...
	fn fmt_file_size(size: u64, human_readable: bool) -> String {
		if !human_readable {
			return size.to_string();
		}

		const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
		let mut size_f = size as f64;
		let mut unit_index = 0;

		while size_f >= 1024.0 && unit_index < UNITS.len() - 1 {
			size_f /= 1024.0;
			unit_index += 1;
		}

		if unit_index == 0 {
			format!("{}{}", size_f as u64, UNITS[unit_index])
		} else {
			format!("{:.1}{}", size_f, UNITS[unit_index])
		}
	}

	// https://doc.rust-lang.org/std/fs/fn.read_dir.html
	fn execute_with_args(args: LsArgs) -> CommandResult {
		// Read directory entries
		let mut entries: Vec<_> = std::fs::read_dir(&args.path)
			.map_err(|e| format!("ls: cannot access '{}': {}", args.path, e))?
			.collect::<Result<Vec<_>, _>>()
			.map_err(|e| format!("ls: {e}"))?;

		// Filter out hidden files (usually dot files) but maybe there's more to
		// consider
		if !args.all {
			entries.retain(|entry| {
				entry
					.file_name()
					.to_str()
					.map(|name| args.all || !name.starts_with("."))
					.unwrap_or(true)
			});
		}

		// Sort entries bcs the order is not guaranteed by `read_dir`
		entries.sort_by_key(|entry| entry.path());
		if args.reverse {
			entries.reverse();
		}

		// Display
		for entry in entries {
			let path = entry.path();
			let file_name = entry.file_name();
			let display_name = file_name.to_string_lossy();

			if args.long {
				// File metadata for -l
				match entry.metadata() {
					Ok(metadata) => {
						// Gives human readable size if args.human_readable
						let file_size = Self::fmt_file_size(
							metadata.len(),
							args.human_readable,
						);

						// TODO: file permissions (no idea how to do this yet)
						// TODO: I would also like to order the output as
						// name - size - permissions
						// but need to fix the formatting for it to work cleanly
						let file_perms = "---------";

						println!("{file_perms} {file_size:>8}  {display_name}");
					}
					Err(_) => {
						println!("?-?-?-?-? {display_name}");
					}
				}
			} else {
				// Basic format just output the name
				let display_path = path.strip_prefix("./").unwrap_or(&path);
				println!("{}", display_path.display());
			}
		}

		Ok(())
	}
}
