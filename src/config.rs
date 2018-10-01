
use std::{
	path::Path,
	fs::File,
	io::{Read, Write, Result as IOResult},
	collections::HashMap
};

use serde_json::{Value, from_str};

const DEFAULT_CONFIG: &'static str = include_str!("default_shell.json");

#[derive(Deserialize)]
pub struct Configuration {
	pub prefix: String,
	pub shell_character: char,
	pub aliases: HashMap<String, String>
}

pub fn create_default_config(path: &Path) -> IOResult<()> {
	let mut output = File::create(path)?;
	output.write_all(DEFAULT_CONFIG.as_bytes())?;
	Ok(())
}

pub fn read_config_file(path: &Path) -> Result<Configuration, String> {
	let mut f = File::open(path.display().to_string()).expect("could not open file");
	let mut contents = String::new();

	f.read_to_string(&mut contents).expect("could not read configuration file");
	
	let result: Configuration = from_str(&contents).expect("could not decode");
	return Ok(result);
}
