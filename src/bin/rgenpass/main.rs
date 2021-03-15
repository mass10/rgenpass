extern crate crossterm;

mod application;
mod generator;
mod keyboard;

/// Application entrypoint.
fn main() {
	// Run application.
	let result = application::run();
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] {}", error);
		return;
	}
}
