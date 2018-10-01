
mod instance;

fn main() {

	let inst = instance::Instance::new();
	match inst {
		Ok(inst) => inst.handle(),
		Err(_) => {}
	}
}
