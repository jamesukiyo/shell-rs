use clap::{Parser, arg};

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(name = "ls")]
#[command(about = "List contents of a directory")]
#[clap(disable_help_flag = true)] // https://github.com/clap-rs/clap/discussions/3715#discussioncomment-7220515
pub struct LsArgs {
	// Allows use of -h elsewhere because otherwise it is used for help
	// So now we just use --help for help
	#[clap(long, action = clap::ArgAction::HelpLong)]
	help: Option<bool>,

	/// Directory path to list
	#[arg(default_value = ".")]
	pub path: String,

	/// Show hidden files
	#[arg(short = 'a', long = "all")]
	pub all: bool,

	/// Use long format (currently no permissions)
	#[arg(short = 'l', long = "long")]
	pub long: bool,

	/// Human readable output, only applies when used with -l
	#[arg(short = 'h', long = "human")]
	pub human_readable: bool,

	/// Reverse the sort order
	#[arg(short = 'r', long = "reverse")]
	pub reverse: bool,
}
