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
	// TODO: more better!
	let (characters_set, width) = match complexity {
		// easy
		0 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 10),
		1 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 13),
		2 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 16),
		3 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 19),
		4 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 22),
		5 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 25),
		6 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 28),
		7 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 31),
		8 => ("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz", 33),
		// low
		9 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 10),
		10 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 13),
		11 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 16),
		12 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 19),
		13 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 22),
		14 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 25),
		15 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 28),
		16 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 31),
		17 => ("+-0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz", 33),
		// complex
		18 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 20),
		19 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 25),
		20 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 30),
		21 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 35),
		22 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 40),
		23 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 45),
		24 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 50),
		25 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 60),
		26 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 65),
		27 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 70),
		28 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 75),
		29 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 80),
		30 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 85),
		31 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 90),
		32 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 91),
		33 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 92),
		34 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 93),
		35 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 94),
		36 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 95),
		37 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 96),
		38 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 97),
		39 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 98),
		40 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 99),
		41 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 100),
		42 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 101),
		43 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 102),
		44 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 103),
		45 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 104),
		46 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 105),
		47 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 106),
		48 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 107),
		49 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 108),
		50 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 109),
		51 => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", 110),
		_ => (" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~", complexity + 59),
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
