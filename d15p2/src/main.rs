use std::collections::HashMap;

fn main() {
	let numbers: Vec<u32> = vec![8, 0, 17, 4, 1, 12];
	let mut number_count: u32 = numbers.len() as u32;
	let mut numbers: HashMap<u32, u32> = numbers
		.iter()
		.enumerate()
		.map(|(index, value)| (*value, index as u32))
		.collect();
	let mut most_recent_number: (u32, u32) = (12, 0);

	while number_count < 30000000 {
		let current_number = most_recent_number.1;
		let current_number_last_found = numbers.entry(current_number).or_insert(number_count);
		let next_number = number_count - *current_number_last_found;
		*current_number_last_found = number_count;
		most_recent_number = (current_number, next_number);
		number_count += 1;
	}

	println!("{}", most_recent_number.0);
}
