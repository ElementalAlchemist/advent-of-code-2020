use std::fs;

#[derive(Eq, PartialEq)]
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
				let row_min_index = if seat_index == 0 { 0 } else { seat_index - 1 };
				let row_max_index = (seat_index + 1).min(seat_col.len() - 1);
				let col_min_index = if seat_col_index == 0 { 0 } else { seat_col_index - 1 };
				let col_max_index = (seat_col_index + 1).min(seating_arrangement.len() - 1);

				let mut occupied_seats: u8 = 0;
				for col in col_min_index..=col_max_index {
					for row in row_min_index..=row_max_index {
						if seating_arrangement[col][row] == PlaceStatus::OccupiedSeat {
							occupied_seats += 1;
						}
					}
				}
				match *seat {
					PlaceStatus::OccupiedSeat => {
						if occupied_seats >= 5 {
							// The rule says 4 but our search includes the seat in question in addition to the adjacent seats
							toggles_to_apply.push((seat_col_index, seat_index));
						}
					}
					PlaceStatus::EmptySeat => {
						if occupied_seats == 0 {
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
		occupied_count += seat_col.iter().filter(|seat| **seat == PlaceStatus::OccupiedSeat).count();
	}
	println!("{}", occupied_count);
}
