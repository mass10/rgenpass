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
fn generate_string(complexity: &str, length: usize) -> String {
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
pub fn generate_password(complexity: u8) -> String {
	// complexity and length
	// TODO: more better!
	let (characters_set, width) = match complexity {
		0 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 10),
		1 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 13),
		2 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 16),
		3 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 19),
		4 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 22),
		5 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 25),
		6 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 28),
		7 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 31),
		8 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 33),
		9 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 10),
		10 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 15),
		11 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 20),
		12 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 25),
		13 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 30),
		14 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 20),
		15 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 25),
		16 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 30),
		17 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 35),
		18 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 40),
		19 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 45),
		20 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 50),
		_ => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 60),
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
