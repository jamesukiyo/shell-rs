use crate::builtin;

pub fn exec_command(input: &str) -> Result<(), String> {
	let parts: Vec<&str> = input.split_whitespace().collect();
	if parts.is_empty() {
		return Ok(());
	}

	let cmd = parts[0];
	let args = &parts[1..];

	if builtin::is_builtin(cmd) {
		builtin::exec_builtin(cmd, args)
	} else {
		Err(format!("Command not found: '{cmd}'"))
	}
}
