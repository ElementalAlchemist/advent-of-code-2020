use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let passports = fs::read_to_string("input.txt")?;
	let passports: Vec<String> = passports.split("\n\n").map(String::from).collect();

	let mut num_valid: u32 = 0;

	for passport in passports.iter() {
		let birth_year_pos = if let Some(field) = passport.find("byr:") {
			field
		} else {
			continue;
		};
		let mut birth_year_iter = passport.chars();
		birth_year_iter.nth(birth_year_pos + 3);
		let mut birth_year: Vec<char> = Vec::new();
		let birth_year: String = loop {
			let next_char = birth_year_iter.next().unwrap_or('\n');
			if next_char.is_whitespace() {
				break birth_year.iter().collect();
			}
			birth_year.push(next_char);
		};
		let birth_year: u32 = if let Ok(year) = birth_year.parse() {
			year
		} else {
			continue;
		};
		if birth_year < 1920 || birth_year > 2002 {
			continue;
		}

		let issue_year_pos = if let Some(field) = passport.find("iyr:") {
			field
		} else {
			continue;
		};
		let mut issue_year_iter = passport.chars();
		issue_year_iter.nth(issue_year_pos + 3);
		let mut issue_year: Vec<char> = Vec::new();
		let issue_year: String = loop {
			let next_char = issue_year_iter.next().unwrap_or('\n');
			if next_char.is_whitespace() {
				break issue_year.iter().collect();
			}
			issue_year.push(next_char);
		};
		let issue_year: u32 = if let Ok(year) = issue_year.parse() {
			year
		} else {
			continue;
		};
		if issue_year < 2010 || issue_year > 2020 {
			continue;
		}

		let expiration_year_pos = if let Some(field) = passport.find("eyr:") {
			field
		} else {
			continue;
		};
		let mut expiration_year_iter = passport.chars();
		expiration_year_iter.nth(expiration_year_pos + 3);
		let mut expiration_year: Vec<char> = Vec::new();
		let expiration_year: String = loop {
			let next_char = expiration_year_iter.next().unwrap_or('\n');
			if next_char.is_whitespace() {
				break expiration_year.iter().collect();
			}
			expiration_year.push(next_char);
		};
		let expiration_year: u32 = if let Ok(year) = expiration_year.parse() {
			year
		} else {
			continue;
		};
		if expiration_year < 2020 || expiration_year > 2030 {
			continue;
		}

		let height_pos = if let Some(field) = passport.find("hgt:") {
			field
		} else {
			continue;
		};
		let mut height_iter = passport.chars();
		height_iter.nth(height_pos + 3);
		let mut height: Vec<char> = Vec::new();
		let height: String = loop {
			let next_char = height_iter.next().unwrap_or('\n');
			if next_char.is_whitespace() {
				break height.iter().collect();
			}
			height.push(next_char);
		};
		if let Some(height_inches) = height.strip_suffix("in") {
			let height_inches: u32 = if let Ok(height_in) = height_inches.parse() {
				height_in
			} else {
				continue;
			};
			if height_inches < 59 || height_inches > 76 {
				continue;
			}
		} else if let Some(height_cm) = height.strip_suffix("cm") {
			let height_cm: u32 = if let Ok(height_cm) = height_cm.parse() {
				height_cm
			} else {
				continue;
			};
			if height_cm < 150 || height_cm > 193 {
				continue;
			}
		} else {
			continue;
		}

		let hair_color_pos = if let Some(color) = passport.find("hcl:") {
			color
		} else {
			continue;
		};
		let mut hair_color_iter = passport.chars();
		hair_color_iter.nth(hair_color_pos + 3);
		let mut hair_color: Vec<char> = Vec::new();
		let hair_color: String = loop {
			let next_char = hair_color_iter.next().unwrap_or('\n');
			if next_char.is_whitespace() {
				break hair_color.iter().collect();
			}
			hair_color.push(next_char);
		};
		let hair_color = if let Some(color) = hair_color.strip_prefix('#') {
			color
		} else {
			continue;
		};
		if hair_color.len() != 6 {
			continue;
		}
		if !hair_color.chars().all(|c| c.is_digit(16)) {
			continue;
		}

		let eye_color_pos = if let Some(color) = passport.find("ecl:") {
			color
		} else {
			continue;
		};
		let mut eye_color_iter = passport.chars();
		eye_color_iter.nth(eye_color_pos + 3);
		let mut eye_color: Vec<char> = Vec::new();
		let eye_color: String = loop {
			let next_char = eye_color_iter.next().unwrap_or('\n');
			if next_char.is_whitespace() {
				break eye_color.iter().collect();
			}
			eye_color.push(next_char);
		};
		if eye_color != "amb"
			&& eye_color != "blu"
			&& eye_color != "brn"
			&& eye_color != "gry"
			&& eye_color != "grn"
			&& eye_color != "hzl"
			&& eye_color != "oth"
		{
			continue;
		}

		let passport_id_pos = if let Some(id) = passport.find("pid:") {
			id
		} else {
			continue;
		};
		let mut passport_id_iter = passport.chars();
		passport_id_iter.nth(passport_id_pos + 3);
		let mut passport_id: Vec<char> = Vec::new();
		let passport_id: String = loop {
			let next_char = passport_id_iter.next().unwrap_or('\n');
			if next_char.is_whitespace() {
				break passport_id.iter().collect();
			}
			passport_id.push(next_char);
		};
		if passport_id.len() != 9 {
			continue;
		}
		if !passport_id.chars().all(|c| c.is_digit(10)) {
			continue;
		}

		num_valid += 1;
	}

	println!("{}", num_valid);

	Ok(())
}
