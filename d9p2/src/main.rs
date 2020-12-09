use std::fs;

const CRACK_TARGET: i64 = 22406676;

fn main() {
	let data = fs::read_to_string("input.txt").expect("Failed to read input file");
	let data: Vec<i64> = data
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(|s| s.parse().expect("Failed to parse numeric input"))
		.collect();

	for (line, _) in data.iter().enumerate() {
		let mut sum: i64 = 0;
		let mut values: Vec<i64> = Vec::new();
		for addend in data.iter().skip(line) {
			sum += *addend;
			values.push(*addend);
			if sum >= CRACK_TARGET {
				break;
			}
		}
		if sum == CRACK_TARGET {
			let mut min = values[0];
			let mut max = values[0];
			for val in values {
				if val < min {
					min = val;
				}
				if val > max {
					max = val;
				}
			}
			println!("{}", min + max);
			break;
		}
	}
}
