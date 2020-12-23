use std::collections::VecDeque;

fn main() {
	let mut cup_order: VecDeque<u64> = VecDeque::from(vec![3, 6, 4, 2, 8, 9, 7, 1, 5]);
	let lowest_cup = *cup_order.iter().min().unwrap();
	let highest_cup = *cup_order.iter().max().unwrap();
	for cup_number in highest_cup + 1..=1_000_000 {
		cup_order.push_back(cup_number);
	}
	let highest_cup = 1_000_000;

	for iteration in 0..10_000_000 {
		if iteration % 1000 == 0 {
			println!("i:{}", iteration);
		}

		let first_cup = cup_order.pop_front().unwrap();
		let next_three_cups = (
			cup_order.pop_front().unwrap(),
			cup_order.pop_front().unwrap(),
			cup_order.pop_front().unwrap(),
		);

		let mut next_cup = first_cup;
		loop {
			next_cup = {
				let next = next_cup - 1;
				if next < lowest_cup {
					highest_cup
				} else {
					next
				}
			};
			if next_cup != next_three_cups.0 && next_cup != next_three_cups.1 && next_cup != next_three_cups.2 {
				break;
			}
		}
		let place_after_cup_at_index = cup_order.iter().position(|x| *x == next_cup).unwrap();
		if place_after_cup_at_index == cup_order.len() {
			cup_order.push_back(next_three_cups.0);
			cup_order.push_back(next_three_cups.1);
			cup_order.push_back(next_three_cups.2);
		} else {
			cup_order.insert(place_after_cup_at_index + 1, next_three_cups.2);
			cup_order.insert(place_after_cup_at_index + 1, next_three_cups.1);
			cup_order.insert(place_after_cup_at_index + 1, next_three_cups.0);
		}
		cup_order.push_back(first_cup);
	}

	let mut cup_iter = cup_order
		.iter()
		.cycle()
		.skip(cup_order.iter().position(|x| *x == 1).unwrap());
	cup_iter.next(); // Skip the 1 value itself
	let first_cup = *cup_iter.next().unwrap();
	let second_cup = *cup_iter.next().unwrap();
	println!("{} * {} = {}", first_cup, second_cup, first_cup * second_cup);
}
