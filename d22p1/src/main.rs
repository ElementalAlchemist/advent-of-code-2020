use std::collections::VecDeque;
use std::fs;

fn main() {
	let (mut player_1_deck, mut player_2_deck) = {
		let input = fs::read_to_string("input.txt").expect("Failed to read input file");
		let mut input_iter = input.split("\n\n");
		let player_1_deck: VecDeque<u64> = input_iter
			.next()
			.unwrap()
			.split('\n')
			.skip(1)
			.map(|s| s.parse().unwrap())
			.collect();
		let player_2_deck: VecDeque<u64> = input_iter
			.next()
			.unwrap()
			.split('\n')
			.filter(|s| !s.is_empty())
			.skip(1)
			.map(|s| s.parse().unwrap())
			.collect();

		(player_1_deck, player_2_deck)
	};

	loop {
		let player_1_card = player_1_deck.pop_front().unwrap();
		let player_2_card = player_2_deck.pop_front().unwrap();
		if player_1_card > player_2_card {
			player_1_deck.push_back(player_1_card);
			player_1_deck.push_back(player_2_card);
		} else {
			player_2_deck.push_back(player_2_card);
			player_2_deck.push_back(player_1_card);
		}

		if player_1_deck.is_empty() || player_2_deck.is_empty() {
			break;
		}
	}

	let winning_deck = if player_1_deck.is_empty() {
		&player_2_deck
	} else {
		&player_1_deck
	};

	let mut multiplier: u64 = 1;
	let mut total: u64 = 0;
	for card in winning_deck.iter().rev() {
		total += card * multiplier;
		multiplier += 1;
	}

	println!("{}", total);
}
