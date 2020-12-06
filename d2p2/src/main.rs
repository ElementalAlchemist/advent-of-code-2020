use std::error::Error;
use std::fs;

struct PasswordCheck {
	first_occurrence: u32,
	second_occurrence: u32,
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
		let first_occurrence: u32 = char_buffer.parse().unwrap();
		char_buffer.clear();

		next_char = char_iter.next().unwrap();
		while next_char != ' ' {
			char_buffer.push(next_char);
			next_char = char_iter.next().unwrap();
		}
		let second_occurrence: u32 = char_buffer.parse().unwrap();
		char_buffer.clear();

		let check_character = char_iter.next().unwrap();
		char_iter.next();
		char_iter.next();
		let password_to_check: String = char_iter.collect();

		PasswordCheck {
			first_occurrence,
			second_occurrence,
			check_character,
			password_to_check,
		}
	}

	fn is_valid(&self) -> bool {
		let mut count: u32 = 0;
		let mut password_iter = self.password_to_check.chars();
		let first_char = loop {
			count += 1;
			let current_char = password_iter.next().unwrap();
			if count == self.first_occurrence {
				break current_char;
			}
		};
		let second_char = loop {
			count += 1;
			let current_char = password_iter.next().unwrap();
			if count == self.second_occurrence {
				break current_char;
			}
		};

		(first_char == self.check_character && second_char != self.check_character)
			|| (first_char != self.check_character && second_char == self.check_character)
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let passwords = fs::read_to_string("input.txt")?;
	let passwords: Vec<String> = passwords
		.split('\n')
		.map(String::from)
		.filter(|x| !x.is_empty())
		.collect();
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
