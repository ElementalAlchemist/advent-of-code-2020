use std::error::Error;
use std::fs;

struct PasswordCheck {
	min_occurrences: u32,
	max_occurrences: u32,
	check_character: char,
	password_to_check: String,
}

impl PasswordCheck {
	fn new(definition: &str) -> PasswordCheck {
		let mut char_buffer = String::new();
		let mut char_iter = definition.chars();
		let mut next_char = char_iter.next().unwrap();
		while next_char != '-' {
			char_buffer.push(next_char);
			next_char = char_iter.next().unwrap();
		}
		let min_occurrences: u32 = char_buffer.parse().unwrap();
		char_buffer.clear();

		next_char = char_iter.next().unwrap();
		while next_char != ' ' {
			char_buffer.push(next_char);
			next_char = char_iter.next().unwrap();
		}
		let max_occurrences: u32 = char_buffer.parse().unwrap();
		char_buffer.clear();

		let check_character = char_iter.next().unwrap();
		char_iter.next();
		char_iter.next();
		let password_to_check: String = char_iter.collect();

		PasswordCheck {
			min_occurrences,
			max_occurrences,
			check_character,
			password_to_check,
		}
	}

	fn is_valid(&self) -> bool {
		let mut count: u32 = 0;
		for pass_char in self.password_to_check.chars() {
			if pass_char == self.check_character {
				count += 1;
			}
		}
		count >= self.min_occurrences && count <= self.max_occurrences
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let passwords = fs::read_to_string("input.txt")?;
	let passwords: Vec<String> = passwords.split('\n').map(String::from).filter(|x| !x.is_empty()).collect();
	let passwords: Vec<PasswordCheck> = passwords.iter().map(|x| PasswordCheck::new(x)).collect();

	let mut success_count: u32 = 0;
	for password in passwords.iter() {
		if password.is_valid() {
			success_count += 1;
		}
	}

	println!("{}", success_count);
	Ok(())
}
