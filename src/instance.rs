
use std::{
	io::{
		stdin,
		stdout,
		Write
	}
};
pub struct Instance {

}

impl Instance {

	pub fn new() -> Result<Instance, ()> {
		Ok(Instance {})
	}

	pub fn handle(&self) {
		loop {
			print!("> ");
			stdout().flush();

			let mut buffer = String::new();
			match stdin().read_line(&mut buffer) {
				Ok(_) => {
					buffer = buffer.trim().to_string();
					if buffer == "" {
						continue;
					}
					println!("{}", buffer);
				},
				Err(err) => println!("Encountered an error while taking from stdin: {}", err)
			}
		}
	}
}
