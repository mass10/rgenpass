use super::generator;

use super::util;
use crossterm::event::KeyEvent;

/// Detects key press.
///
/// # Returns
/// Pressed key identifier if exists.
fn detect_key() -> std::result::Result<Option<crossterm::event::Event>, std::boxed::Box<dyn std::error::Error>> {
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

	// Make screen raw mode.
	if false {
		crossterm::terminal::enable_raw_mode()?;
	}

	println!("(Press [Enter] or [Space] to generate random password.)");

	let mut current_complexity = 0;

	let mut time_keeper = util::TimeKeeper::new();

	// Main event loop for key press.
	loop {
		// Handle termination.
		if time_keeper.is_over() {
			break;
		}

		// Detect key press.
		let result = detect_key()?;
		if result.is_none() {
			continue;
		}
		let key = result.unwrap();

		time_keeper.start();

		match key {
			// [Ctrl][C] to quit.
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => break,
			// [Enter]
			Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => {
				current_complexity = current_complexity + 1;
				println!("{}", generator::generate_password(current_complexity / 3));
			}
			// [Enter]
			Event::Key(KeyEvent {
				code: KeyCode::Char(' '),
				modifiers: KeyModifiers::NONE,
			}) => {
				current_complexity = current_complexity + 1;
				println!("{}", generator::generate_password(current_complexity / 3));
			}
			// Else
			_ => break,
		}
	}

	println!();

	return Ok(());
}
