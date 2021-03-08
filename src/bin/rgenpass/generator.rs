extern crate rand;

/// パスワードを生成します。
fn generate_string(complexity: &str, length: u32) -> String {
	use rand::Rng;

	// 文字列を文字ベクターにばらす
	let chars: Vec<char> = complexity.chars().collect();
	// 文字数
	let len = chars.len();
	// 乱数生成器
	let mut generator = rand::thread_rng();
	// 応答
	let mut response = "".to_string();

	// 要求された数で繰り返し
	for _ in 0..length {
		// ランダムな文字
		let letter_position = generator.gen::<u8>() % len as u8;
		let letter = chars[letter_position as usize];

		response.push(letter);
	}

	return response;
}

#[allow(unused)]
pub fn generate_password_c() -> String {
	return generate_string("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 10);
}

#[allow(unused)]
pub fn generate_password_cc() -> String {
	return generate_string("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 10);
}

#[allow(unused)]
pub fn generate_password_ccc() -> String {
	return generate_string("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 10);
}

pub fn generate_password(current_complexity: u8) -> Result<(), Box<dyn std::error::Error>> {
	// complexity, and width
	let (characters_set, width) = match current_complexity {
		0 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 10),
		1 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 15),
		2 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 20),
		3 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 25),
		4 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890", 30),

		5 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 10),
		6 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 15),
		7 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 20),
		8 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 25),
		9 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=+_", 30),

		10 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 20),
		11 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 25),
		12 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 30),
		13 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 35),
		14 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 40),
		15 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 45),
		16 => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 50),
		_ => ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890-=^~\\|@`[{{;+:*]}}!\"#$%&'(),./<>?_ ", 60),
	};

	// 文字列を生成
	let password = generate_string(characters_set, width);

	println!("{} ({})", password, password.len());

	return Ok(());
}
