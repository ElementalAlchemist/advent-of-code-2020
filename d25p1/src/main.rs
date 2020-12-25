use std::fs;

fn main() {
	let (card_key, door_key): (u64, u64) = {
		let input = fs::read_to_string("input.txt").expect("Failed to read input file");
		let mut input_iter = input.split('\n');
		let card_key = input_iter.next().unwrap();
		let door_key = input_iter.next().unwrap();
		(card_key.parse().unwrap(), door_key.parse().unwrap())
	};

	let mut card_loop_size: u64 = 0;
	let mut card_value: u64 = 1;
	while card_value != card_key {
		card_value *= 7;
		card_value %= 20201227;
		card_loop_size += 1;
	}

	let mut encryption_key: u64 = 1;
	for _ in 0..card_loop_size {
		encryption_key *= door_key;
		encryption_key %= 20201227;
	}

	println!("{}", encryption_key);
}
