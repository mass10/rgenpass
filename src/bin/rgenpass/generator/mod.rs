//!
//! mod generator
//!

extern crate rand;

/// Validate string for password.
///
/// ### Returns
/// `true` if valid
fn is_valid_password(s: &str) -> bool {
	if s.starts_with(" ") {
		return false;
	}
	if s.ends_with(" ") {
		return false;
	}

	if s.starts_with("\"") {
		return false;
	}

	if s.starts_with("'") {
		return false;
	}

	return true;
}

/// Generates a string.
///
/// ### Arguments
/// `complexity` String complexity
/// `length` Required length
///
/// ### Returns
/// a new string
fn generate_string(complexity: &str, length: u8) -> String {
	use rand::Rng;

	// generator
	let mut generator = rand::thread_rng();

	let chars: Vec<char> = complexity.chars().collect();
	let len = chars.len();
	let mut response = "".to_string();

	for _ in 0..length {
		// generate random char
		let letter_position = generator.gen::<usize>() % len;
		let letter = chars[letter_position];

		// append to response
		response.push(letter);
	}

	return response;
}

/// Generates a new password. (WIP)
///
/// ### Arguments
/// `complexity` Password complexity
fn generate_password(complexity: u8) -> String {
	// complexity and length
	let (characters_set, width) = match complexity {
		// simple
		0..=10 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 10 + (complexity * 3)),
		// low
		11..=21 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 10 + ((complexity - 11) * 3)),
		// complex
		_ => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 10 + ((complexity - 22) * 3)),
	};

	loop {
		// Generate a new password
		let password = generate_string(characters_set, width);

		// validation
		if !is_valid_password(&password) {
			continue;
		}

		// It's OK.
		return password;
	}
}

///
/// Complexity controller
///
struct ComplexityController {
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
	pub fn get_current_complexity(&mut self) -> u8 {
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

///
/// Password generator.
///
pub struct PasswordGenerator {
	complexity_controller: ComplexityController,
}

impl PasswordGenerator {
	/// Returns a new instance of `PasswordGenerator`.
	///
	/// ### Returns
	/// A new instance of `PasswordGenerator`.
	pub fn new() -> PasswordGenerator {
		return PasswordGenerator { complexity_controller: ComplexityController::new() };
	}

	/// Shows new password.
	pub fn request(&mut self) {
		println!("{}", generate_password(self.complexity_controller.get_current_complexity()))
	}
}
