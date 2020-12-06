use std::error::Error;
use std::fs;

struct Seat {
	row: u32,
	pos_in_row: u32,
}

impl Seat {
	fn from_binary_specifier(spec: &str) -> Self {
		let mut row: u32 = 0;
		let mut pos_in_row: u32 = 0;
		let mut seat_chars = spec.chars();

		if seat_chars.next().unwrap() == 'B' {
			row += 64;
		}
		if seat_chars.next().unwrap() == 'B' {
			row += 32;
		}
		if seat_chars.next().unwrap() == 'B' {
			row += 16;
		}
		if seat_chars.next().unwrap() == 'B' {
			row += 8;
		}
		if seat_chars.next().unwrap() == 'B' {
			row += 4;
		}
		if seat_chars.next().unwrap() == 'B' {
			row += 2;
		}
		if seat_chars.next().unwrap() == 'B' {
			row += 1;
		}
		if seat_chars.next().unwrap() == 'R' {
			pos_in_row += 4;
		}
		if seat_chars.next().unwrap() == 'R' {
			pos_in_row += 2;
		}
		if seat_chars.next().unwrap() == 'R' {
			pos_in_row += 1;
		}

		Seat { row, pos_in_row }
	}

	fn seat_id(&self) -> u32 {
		self.row * 8 + self.pos_in_row
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let passes = fs::read_to_string("input.txt")?;
	let passes: Vec<String> = passes.split('\n').map(String::from).filter(|s| !s.is_empty()).collect();

	let mut highest_id: u32 = 0;

	for pass in passes.iter() {
		let seat = Seat::from_binary_specifier(pass);
		let seat_id = seat.seat_id();
		if seat_id > highest_id {
			highest_id = seat_id;
		}
	}

	println!("{}", highest_id);

	Ok(())
}
