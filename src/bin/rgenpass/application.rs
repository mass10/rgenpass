use crossterm::event::KeyEvent;

use super::generator;
use super::keyboard;

///
/// Complexity controller
///
pub struct ComplexityController {
	/// Start time
	start: std::time::Instant,
	/// Complexity
	complexity: u8,
}

impl ComplexityController {
	/// Returns a new instance.
	///
	/// ### Returns
	/// A new instance of [ComplexityTimeKeeper]
	pub fn new() -> ComplexityController {
		return ComplexityController { start: std::time::Instant::now(), complexity: 0 };
	}

	/// Increment innternal value.
	fn increment(&mut self) {
		// prevent MAX overflow.
		self.complexity = std::cmp::max(self.complexity, self.complexity + 1);
	}

	/// Decrement innternal value.
	fn decrement(&mut self) {
		// prevent MIN overflow.
		self.complexity = std::cmp::min(self.complexity, self.complexity - 1);
	}

	/// Refresh complexity.
	///
	/// ### Returns
	/// The current complexity.
	pub fn refresh(&mut self) -> u8 {
		// elapsed time
		let current_time = std::time::Instant::now();
		let erapsed = current_time - self.start;

		// Reset internal timeer.
		self.start = current_time;

		if erapsed.as_millis() < 180 {
			// Up
			self.increment();
			return self.complexity;
		}

		if erapsed.as_millis() < 250 {
			// No updates.
			return self.complexity;
		}

		// Down
		self.decrement();
		return self.complexity;
	}
}

/// Run application.
pub fn run() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	use crossterm::event::{Event, KeyCode, KeyModifiers};

	// In fact, any key is applied.
	println!("(Press [Enter] or [Space] to generate random password.)");

	// Keyboard queue
	let mut keyboard_queue = keyboard::KeyboardQueue::new();

	// complexity time keeper
	let mut complexity_controller = ComplexityController::new();

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
			Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => {
				println!("{}", generator::generate_password(complexity_controller.refresh()));
			}
			// [Space]
			Event::Key(KeyEvent { code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE }) => {
				println!("{}", generator::generate_password(complexity_controller.refresh()))
			}
			// Else (any key is applied)
			_ => {
				println!("{}", generator::generate_password(complexity_controller.refresh()))
			}
		}
	}

	return Ok(());
}
