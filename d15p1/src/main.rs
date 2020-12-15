fn main() {
	let mut numbers: Vec<u32> = vec![8, 0, 17, 4, 1, 12];
	while numbers.len() < 2020 {
		let mut number_iter = numbers.iter().rev();
		let previous_number = *number_iter.next().unwrap();

		let mut iter_count: u32 = 0;
		let mut found = false;
		for number in number_iter {
			iter_count += 1;
			if *number == previous_number {
				found = true;
				break;
			}
		}
		if found {
			numbers.push(iter_count);
		} else {
			numbers.push(0);
		}
	}

	println!("{}", numbers[2019]);
}
