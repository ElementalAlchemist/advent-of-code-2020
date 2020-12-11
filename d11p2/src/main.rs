use std::fs;

#[derive(Clone, Copy, Eq, PartialEq)]
enum PlaceStatus {
	OccupiedSeat,
	EmptySeat,
	Floor,
}

impl PlaceStatus {
	fn from_char(place_type: char) -> Self {
		match place_type {
			'#' => Self::OccupiedSeat,
			'L' => Self::EmptySeat,
			'.' => Self::Floor,
			_ => panic!("Invalid place type: {}", place_type),
		}
	}

	fn is_seat(&self) -> bool {
		*self != Self::Floor
	}
}

fn is_visible_seat_with_processing(visible_seats: &mut Vec<PlaceStatus>, seat: &PlaceStatus) -> bool {
	if seat.is_seat() {
		visible_seats.push(*seat);
		true
	} else {
		false
	}
}

fn main() {
	let seating_arrangement = fs::read_to_string("input.txt").expect("Failed to read input file");
	let seating_arrangement: Vec<String> = seating_arrangement
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(String::from)
		.collect();
	let mut seating_arrangement: Vec<Vec<PlaceStatus>> = seating_arrangement
		.iter()
		.map(|seats| seats.chars().map(PlaceStatus::from_char).collect())
		.collect();

	loop {
		let mut toggles_to_apply: Vec<(usize, usize)> = Vec::new();
		for (seat_col_index, seat_col) in seating_arrangement.iter().enumerate() {
			for (seat_index, seat) in seat_col.iter().enumerate() {
				if *seat == PlaceStatus::Floor {
					continue;
				}

				let mut visible_seats: Vec<PlaceStatus> = Vec::new();
				// left
				for visible_seat_col in seating_arrangement[0..seat_col_index].iter().rev() {
					if is_visible_seat_with_processing(&mut visible_seats, &visible_seat_col[seat_index]) {
						break;
					}
				}
				// right
				for visible_seat_col in seating_arrangement.iter().skip(seat_col_index + 1) {
					if is_visible_seat_with_processing(&mut visible_seats, &visible_seat_col[seat_index]) {
						break;
					}
				}
				// up
				for visible_seat in seat_col[0..seat_index].iter().rev() {
					if is_visible_seat_with_processing(&mut visible_seats, visible_seat) {
						break;
					}
				}
				// down
				for visible_seat in seat_col.iter().skip(seat_index + 1) {
					if is_visible_seat_with_processing(&mut visible_seats, visible_seat) {
						break;
					}
				}
				// up/left
				let mut col = seat_col_index;
				let mut row = seat_index;
				while col > 0 && row > 0 {
					col -= 1;
					row -= 1;
					let seat = &seating_arrangement[col][row];
					if seat.is_seat() {
						visible_seats.push(*seat);
						break;
					}
				}

				// up/right
				let mut col = seat_col_index;
				let mut row = seat_index;
				while col < seating_arrangement.len() - 1 && row > 0 {
					col += 1;
					row -= 1;
					let seat = &seating_arrangement[col][row];
					if seat.is_seat() {
						visible_seats.push(*seat);
						break;
					}
				}

				// down/left
				let mut col = seat_col_index;
				let mut row = seat_index;
				while col > 0 && row < seating_arrangement[col].len() - 1 {
					col -= 1;
					row += 1;
					let seat = &seating_arrangement[col][row];
					if seat.is_seat() {
						visible_seats.push(*seat);
						break;
					}
				}

				// down/right
				let mut col = seat_col_index;
				let mut row = seat_index;
				while col < seating_arrangement.len() - 1 && row < seating_arrangement[col].len() - 1 {
					col += 1;
					row += 1;
					let seat = &seating_arrangement[col][row];
					if seat.is_seat() {
						visible_seats.push(*seat);
						break;
					}
				}

				let visible_occupied_seats = visible_seats
					.iter()
					.filter(|seat| **seat == PlaceStatus::OccupiedSeat)
					.count();
				match *seat {
					PlaceStatus::OccupiedSeat => {
						if visible_occupied_seats >= 5 {
							toggles_to_apply.push((seat_col_index, seat_index));
						}
					}
					PlaceStatus::EmptySeat => {
						if visible_occupied_seats == 0 {
							toggles_to_apply.push((seat_col_index, seat_index));
						}
					}
					PlaceStatus::Floor => unreachable!(),
				}
			}
		}
		if toggles_to_apply.is_empty() {
			break;
		}
		for (col, row) in toggles_to_apply.iter() {
			seating_arrangement[*col][*row] = match seating_arrangement[*col][*row] {
				PlaceStatus::OccupiedSeat => PlaceStatus::EmptySeat,
				PlaceStatus::EmptySeat => PlaceStatus::OccupiedSeat,
				PlaceStatus::Floor => panic!("Attempted to toggle the floor"),
			};
		}
		toggles_to_apply.clear();
	}

	let mut occupied_count: usize = 0;
	for seat_col in seating_arrangement.iter() {
		occupied_count += seat_col
			.iter()
			.filter(|seat| **seat == PlaceStatus::OccupiedSeat)
			.count();
	}
	println!("{}", occupied_count);
}
