mod builtin;
mod exec;
mod prompt;
mod shell;

use shell::Shell;

fn main() {
	println!(" ");
	println!("Welcome to shell-rs 🦀 by github/jamesukiyo!");
	println!("Type 'help' at any time for a list of supported commands.");
	println!(" ");
	println!("A config file can be found at ~/.config/shell-rs/config.toml");
	println!(
		"You can change your emoji, name, separator, prompt character and more :)"
	);
	let mut shell = Shell::new();
	shell.run();
}
