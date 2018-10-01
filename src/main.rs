
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod instance;
mod config;

fn main() {

	let inst = instance::Instance::new();
	match inst {
		Ok(inst) => inst.handle(),
		Err(err) => match err {
			instance::InstanceErrors::CouldNotFindHomeDir => println!("could not find home directory."),
			instance::InstanceErrors::CouldNotParseConfig => println!("could not parse configuration.")
		}
	}
}
