use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let passports = fs::read_to_string("input.txt")?;
	let passports: Vec<String> = passports.split("\n\n").map(String::from).collect();

	let mut valid_count: u32 = 0;

	for passport in passports.iter() {
		if passport.contains("byr:")
			&& passport.contains("iyr:")
			&& passport.contains("eyr:")
			&& passport.contains("hgt:")
			&& passport.contains("hcl:")
			&& passport.contains("ecl:")
			&& passport.contains("pid:")
		{
			valid_count += 1;
		}
	}

	println!("{}", valid_count);

	Ok(())
}
