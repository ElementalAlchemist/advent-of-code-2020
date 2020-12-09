use std::collections::VecDeque;
use std::fs;

fn main() {
	let data = fs::read_to_string("input.txt").expect("Failed to read input file");
	let data: Vec<i64> = data
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(|s| s.parse().expect("Failed to parse numeric input"))
		.collect();

	let mut data_iter = data.iter();
	let mut prev_nums: VecDeque<i64> = VecDeque::new();
	for _ in 0..25 {
		prev_nums.push_back(*data_iter.next().unwrap());
	}
	while let Some(value) = data_iter.next() {
		let mut found_addend = false;
		for possible_addend in prev_nums.iter() {
			if let Some(other_addend) = prev_nums.iter().find(|x| **x == (value - possible_addend)) {
				if possible_addend != other_addend {
					found_addend = true;
				}
			}
		}
		if !found_addend {
			println!("{}", value);
			break;
		}
		prev_nums.pop_front();
		prev_nums.push_back(*value);
	}
}
