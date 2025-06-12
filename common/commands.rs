// Result type for all commands
pub type CommandResult = Result<(), String>;

// Trait for all commands
pub trait ShellCommand {
	// Execute the command with the given arguments
	fn execute(args: &[&str]) -> CommandResult;

	// Get the command name
	fn name() -> &'static str;
}
