// Helper to get HOME directory
pub fn get_home_dir() -> Result<String, String> {
	std::env::var("HOME")
		.map_err(|_| "HOME environment variable not set".to_string())
}
