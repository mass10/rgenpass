//!
//! application entrypoint.
//!

extern crate crossterm;

mod application;
mod generator;
mod io;

/// Application entrypoint.
fn main() {
	// Run application.
	let result = application::Application::new().run();
	if result.is_err() {
		let error = result.err().unwrap();
		println!("[ERROR] {}", error);
		return;
	}
}
