use std::fs;

fn main() {
	let input = fs::read_to_string("input.txt").expect("Failed to read input file");
	let mut input_lines = input.split('\n').skip(1); // Skip the time line this time
	let buses: Vec<Option<u128>> = input_lines
		.next()
		.unwrap()
		.split(',')
		.map(|s| if s == "x" { None } else { Some(s.parse().unwrap()) })
		.collect();

	let mut time_jump: u128 = 23;
	let mut last_time_jump_index: usize = 0;
	let mut time: u128 = 0;
	'time: loop {
		time += time_jump;
		for (bus_index, bus_id) in buses.iter().enumerate() {
			if let Some(bus) = bus_id {
				if ((time + (bus_index as u128)) % *bus) == 0 {
					if bus_index > last_time_jump_index {
						last_time_jump_index = bus_index;
						time_jump *= *bus;
					}
				} else {
					continue 'time;
				}
			}
		}
		break;
	}
	println!("{}", time);
}
