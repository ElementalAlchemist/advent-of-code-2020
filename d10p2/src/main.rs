use std::fs;

fn main() {
	let joltages = fs::read_to_string("input.txt").expect("Failed to read input file");
	let mut joltages: Vec<u32> = joltages
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(|s| s.parse().unwrap())
		.collect();
	joltages.sort_unstable();

	let joltage_count = joltages.len();
	let mut skippable: Vec<u32> = Vec::with_capacity(joltage_count);

	for (joltage_index, _) in joltages.iter().enumerate() {
		let lower_index = if joltage_index < 2 { 0 } else { joltage_index - 2 };
		let less_index = joltage_index;
		let more_index = joltage_index + 1;
		let upper_index = if joltage_index + 2 >= joltage_count {
			joltage_count - 1
		} else {
			joltage_index + 2
		};

		let lower_joltages = &joltages[lower_index..less_index];
		let higher_joltages = &joltages[more_index..=upper_index];
		let mut skips: u32 = 0;
		for low_joltage in lower_joltages.iter() {
			for high_joltage in higher_joltages.iter() {
				if *high_joltage - *low_joltage <= 3 {
					skips += 1;
				}
			}
		}
		if lower_joltages.len() < 2 {
			for high_joltage in higher_joltages.iter() {
				if *high_joltage <= 3 {
					skips += 1;
				}
			}
		}
		skippable.push(skips);
	}

	println!("{:?}", joltages);
	println!("{:?}", skippable);

	let mut combination_count: u64 = 1; // Start with the combination with all adapters
	let mut skip_run: Vec<u32> = Vec::with_capacity(3);
	for skip_ways in skippable.iter() {
		if *skip_ways == 0 {
			if !skip_run.is_empty() {
				if skip_run == vec![1] {
					combination_count *= 2;
				} else if skip_run == vec![2, 2] {
					combination_count *= 4;
				} else if skip_run == vec![2, 2, 2] {
					combination_count *= 7;
				} else if skip_run == vec![2, 3, 2] {
					combination_count *= 7;
				} else {
					panic!(
						"The skip combination {:?} isn't possible with a joltage adapter limit of 3 jolts",
						skip_run
					);
				}
				skip_run.clear();
			}
			continue;
		}
		skip_run.push(*skip_ways);
	}
	println!("{}", combination_count);
}
