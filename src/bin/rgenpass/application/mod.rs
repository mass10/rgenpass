//!
//! Application core implementation.
//!

use crossterm::event::KeyEvent;

use super::generator;
use crate::io::keyboard;

///
/// Application definition.
///
pub struct Application;

impl Application {
	/// Returns a new instance of [Application].
	pub fn new() -> Application {
		return Application {};
	}

	/// Run application.
	pub fn run(&self) -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
		use crossterm::event::{Event, KeyCode, KeyModifiers};

		// In fact, any key is applied.
		println!("(Press [Enter] or [Space] to generate random password.)");

		// Keyboard queue
		let mut keyboard_queue = keyboard::KeyboardQueue::new();

		// password generator
		let mut generator = generator::PasswordGenerator::new();

		// Main event loop for key press.
		loop {
			// Detect key press.
			let result = keyboard_queue.pop()?;
			if result.is_none() {
				// shutdown
				break;
			}
			let key = result.unwrap();

			match key {
				// [Ctrl][C] to quit.
				Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL }) => break,
				// [Q] to quit.
				Event::Key(KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::NONE }) => break,
				// [Enter]
				Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => generator.request(),
				// [Space]
				Event::Key(KeyEvent { code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE }) => generator.request(),
				// Else (any key is applied)
				_ => generator.request(),
			}
		}

		return Ok(());
	}
}
