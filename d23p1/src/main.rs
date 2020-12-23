use std::collections::VecDeque;

fn main() {
	let mut cup_order: Vec<u8> = vec![3, 6, 4, 2, 8, 9, 7, 1, 5];
	let lowest_cup = *cup_order.iter().min().unwrap();
	let highest_cup = *cup_order.iter().max().unwrap();

	for _ in 0..100 {
		let mut cups: VecDeque<u8> = cup_order.iter().copied().collect();
		let first_cup = cups.pop_front().unwrap();
		let next_three_cups = (
			cups.pop_front().unwrap(),
			cups.pop_front().unwrap(),
			cups.pop_front().unwrap(),
		);

		let mut next_cup = first_cup;
		let place_after_cup_at_index = loop {
			next_cup = {
				let next = next_cup - 1;
				if next < lowest_cup {
					highest_cup
				} else {
					next
				}
			};
			if let Some(pos) = cups.iter().position(|x| *x == next_cup) {
				break pos;
			}
		};
		if place_after_cup_at_index >= cups.len() {
			cups.push_back(next_three_cups.0);
			cups.push_back(next_three_cups.1);
			cups.push_back(next_three_cups.2);
		} else {
			cups.insert(place_after_cup_at_index + 1, next_three_cups.2);
			cups.insert(place_after_cup_at_index + 1, next_three_cups.1);
			cups.insert(place_after_cup_at_index + 1, next_three_cups.0);
		}
		cups.push_back(first_cup);

		cup_order = cups.iter().copied().collect();
	}

	let mut resulting_order = String::new();
	for cup in cup_order.iter() {
		resulting_order.extend(cup.to_string().chars());
	}
	println!("{}", resulting_order);
}
