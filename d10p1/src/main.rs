use std::fs;

fn main() {
	let joltages = fs::read_to_string("input.txt").expect("Failed to read input file");
	let mut joltages: Vec<u32> = joltages
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(|s| s.parse().unwrap())
		.collect();

	joltages.sort_unstable();
	let mut one_difference: usize = 0;
	let mut three_difference: usize = 1; // The last adapter to the device is always a jump of 3
	let mut previous_joltage: u32 = 0; // Start with the outlet's joltage
	for joltage in joltages.iter() {
		let joltage = *joltage;
		if joltage - previous_joltage == 1 {
			one_difference += 1;
		} else if joltage - previous_joltage == 3 {
			three_difference += 1;
		}
		previous_joltage = joltage;
	}
	println!("{}", one_difference * three_difference);
}
