use std::collections::HashMap;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinates {
	x: i32,
	y: i32,
	z: i32,
}

impl Coordinates {
	fn new(x: i32, y: i32, z: i32) -> Self {
		Self { x, y, z }
	}

	fn get_adjusted(&self, x_shift: i32, y_shift: i32, z_shift: i32) -> Self {
		let x = self.x + x_shift;
		let y = self.y + y_shift;
		let z = self.z + z_shift;
		Self { x, y, z }
	}
}

fn cycle_cubes(start_cubes: &HashMap<Coordinates, bool>) -> HashMap<Coordinates, bool> {
	let mut changes: HashMap<Coordinates, bool> = HashMap::new();
	let coords_to_check: Vec<Coordinates> = start_cubes
		.keys()
		.map(|coords| {
			let mut all_coords = Vec::with_capacity(9);
			for x_change in -1..=1 {
				for y_change in -1..=1 {
					for z_change in -1..=1 {
						all_coords.push(coords.get_adjusted(x_change, y_change, z_change));
					}
				}
			}
			all_coords
		})
		.flatten()
		.collect();

	for coords in coords_to_check.iter() {
		let mut active_neighbors: u8 = 0;
		for x_change in -1..=1 {
			for y_change in -1..=1 {
				for z_change in -1..=1 {
					if x_change == 0 && y_change == 0 && z_change == 0 {
						continue; // Omit the cube itself from its neighbors
					}
					let check_cube_active = start_cubes
						.get(&coords.get_adjusted(x_change, y_change, z_change))
						.unwrap_or(&false);
					if *check_cube_active {
						active_neighbors += 1;
					}
				}
			}
		}
		let active = start_cubes.get(coords).unwrap_or(&false);
		if *active {
			if active_neighbors != 2 && active_neighbors != 3 {
				changes.insert(coords.clone(), false);
			}
		} else if active_neighbors == 3 {
			changes.insert(coords.clone(), true);
		}
	}
	changes
}

fn main() {
	let cubes = fs::read_to_string("input.txt").expect("Failed to read input file");
	let mut cubes: HashMap<Coordinates, bool> = {
		let mut cube_map = HashMap::new();
		for (col_index, col) in cubes.split('\n').filter(|s| !s.is_empty()).enumerate() {
			for (row_index, row_val) in col.chars().enumerate() {
				cube_map.insert(Coordinates::new(row_index as i32, col_index as i32, 0), row_val == '#');
			}
		}
		cube_map
	};

	for _ in 0..6 {
		let changed_cubes: HashMap<Coordinates, bool> = cycle_cubes(&cubes);
		for (coords, new_state) in changed_cubes {
			let state = cubes.entry(coords).or_default();
			*state = new_state;
		}
	}

	println!("{}", cubes.values().filter(|active| **active).count());
}
