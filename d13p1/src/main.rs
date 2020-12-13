use std::fs;

fn main() {
	let input = fs::read_to_string("input.txt").expect("Failed to read input file");
	let mut bus_data = input.split('\n').filter(|s| !s.is_empty());

	let current_time: u32 = bus_data.next().unwrap().parse().unwrap();
	let bus_list = bus_data.next().unwrap();
	let bus_list: Vec<u32> = bus_list
		.split(',')
		.filter(|s| *s != "x")
		.map(|s| s.parse().unwrap())
		.collect();

	let mut lowest_value: Option<(u32, u32)> = None;
	for bus in bus_list.iter() {
		let wait_time = bus - (current_time % bus);
		if let Some((val, _)) = &lowest_value {
			if wait_time < *val {
				lowest_value = Some((wait_time, *bus));
			}
		} else {
			lowest_value = Some((wait_time, *bus));
		}
	}
	let answer = lowest_value.unwrap().0 * lowest_value.unwrap().1;
	println!("{}", answer);
}
