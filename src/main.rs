
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
		Err(_) => {}
	}
}
