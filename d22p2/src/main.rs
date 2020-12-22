use std::collections::{HashSet, VecDeque};
use std::fs;

enum Player {
	One,
	Two,
}

fn play_game(mut player_1_deck: VecDeque<u64>, mut player_2_deck: VecDeque<u64>) -> (Player, VecDeque<u64>) {
	let mut previous_decks: HashSet<(VecDeque<u64>, VecDeque<u64>)> = HashSet::new();

	loop {
		if previous_decks.contains(&(player_1_deck.clone(), player_2_deck.clone())) {
			break (Player::One, player_1_deck);
		}
		previous_decks.insert((player_1_deck.clone(), player_2_deck.clone()));

		let player_1_card = player_1_deck.pop_front().unwrap();
		let player_2_card = player_2_deck.pop_front().unwrap();

		if player_1_card <= player_1_deck.len() as u64 && player_2_card <= player_2_deck.len() as u64 {
			let mut player_1_new_deck: VecDeque<u64> = VecDeque::with_capacity(player_1_card as usize);
			let mut player_2_new_deck: VecDeque<u64> = VecDeque::with_capacity(player_2_card as usize);
			let mut player_1_deck_iter = player_1_deck.iter();
			let mut player_2_deck_iter = player_2_deck.iter();

			for _ in 0..player_1_card {
				player_1_new_deck.push_back(*player_1_deck_iter.next().unwrap());
			}
			for _ in 0..player_2_card {
				player_2_new_deck.push_back(*player_2_deck_iter.next().unwrap());
			}

			match play_game(player_1_new_deck, player_2_new_deck).0 {
				Player::One => {
					player_1_deck.push_back(player_1_card);
					player_1_deck.push_back(player_2_card);
				}
				Player::Two => {
					player_2_deck.push_back(player_2_card);
					player_2_deck.push_back(player_1_card);
				}
			}
		} else if player_1_card > player_2_card {
			player_1_deck.push_back(player_1_card);
			player_1_deck.push_back(player_2_card);
		} else {
			player_2_deck.push_back(player_2_card);
			player_2_deck.push_back(player_1_card);
		}

		if player_1_deck.is_empty() {
			break (Player::Two, player_2_deck);
		}
		if player_2_deck.is_empty() {
			break (Player::One, player_1_deck);
		}
	}
}

fn main() {
	let (player_1_deck, player_2_deck) = {
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

	let winning_deck = play_game(player_1_deck, player_2_deck).1;

	let mut multiplier: u64 = 1;
	let mut total: u64 = 0;
	for card in winning_deck.iter().rev() {
		total += card * multiplier;
		multiplier += 1;
	}

	println!("{}", total);
}
