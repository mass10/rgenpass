use crossterm::event::KeyEvent;

use super::generator;
use super::util;

/// Detects key press.
///
/// ### Returns
/// Pressed key identifier if exists.
fn detect_key_press() -> std::result::Result<Option<crossterm::event::Event>, std::boxed::Box<dyn std::error::Error>> {
	// Detect some key pressed.
	let result = crossterm::event::poll(std::time::Duration::from_millis(10))?;
	if !result {
		return Ok(None);
	}

	// Return the next key.
	let key = crossterm::event::read()?;

	return Ok(Some(key));
}

/// Run application.
pub fn run() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	use crossterm::event::{Event, KeyCode, KeyModifiers};

	// In fact, any key is applied.
	println!("(Press [Enter] or [Space] to generate random password.)");

	// time keeper
	let mut time_keeper = util::TimeKeeper::new();
	// complexity time keeper
	let mut complexity_controller = util::ComplexityController::new();

	// Main event loop for key press.
	loop {
		// Handle termination.
		if time_keeper.is_timed_out() {
			break;
		}

		// Detect key press.
		let result = detect_key_press()?;
		if result.is_none() {
			continue;
		}
		let key = result.unwrap();

		// Once any key pressed, time keeper starts.
		time_keeper.start();

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
