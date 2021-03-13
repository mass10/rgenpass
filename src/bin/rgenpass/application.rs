use std::cmp::max;

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

fn fix(value: i8) -> i8 {
	return max(0, value);
}

/// Run application.
pub fn run() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
	use crossterm::event::{Event, KeyCode, KeyModifiers};

	// Make screen raw mode.
	if false {
		crossterm::terminal::enable_raw_mode()?;
	}

	println!("(Press [Enter] or [Space] to generate random password.)");

	// password complexity
	let mut current_complexity = 0;

	// time keeper
	let mut time_keeper = util::TimeKeeper::new();

	let mut complexity_time_keeper = util::ComplexityTimeKeeper::new();

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

		// let current_time = std::time::Instant::now();

		match key {
			// [Ctrl][C] to quit.
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => break,
			// [Enter]
			Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => {
				current_complexity = fix(current_complexity + complexity_time_keeper.test());
				println!("{}", generator::generate_password(current_complexity));
			}
			// [Enter]
			Event::Key(KeyEvent {
				code: KeyCode::Char(' '),
				modifiers: KeyModifiers::NONE,
			}) => {
				current_complexity = fix(current_complexity + complexity_time_keeper.test());
				println!("{}", generator::generate_password(current_complexity));
			}
			// Else
			_ => break,
		}
	}

	println!();

	return Ok(());
}
