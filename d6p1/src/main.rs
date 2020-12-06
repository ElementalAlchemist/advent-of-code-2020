use std::error::Error;
use std::fs;

fn count_true(array: &[bool]) -> usize {
	let mut count: usize = 0;
	for val in array.iter() {
		if *val {
			count += 1;
		}
	}
	count
}

fn main() -> Result<(), Box<dyn Error>> {
	let group_answer_data = fs::read_to_string("input.txt")?;
	let group_answer_data: Vec<Vec<String>> = group_answer_data
		.split("\n\n")
		.map(|s| s.split("\n").map(String::from).collect())
		.collect();

	let mut total_yes: usize = 0;
	for group_answer_set in group_answer_data.iter() {
		let mut answered_yes: [bool; 26] = [false; 26];
		for answer_set in group_answer_set.iter() {
			for answer in answer_set.chars() {
				answered_yes[answer as usize - 'a' as usize] = true;
			}
		}
		total_yes += count_true(&answered_yes);
	}

	println!("{}", total_yes);

	Ok(())
}
