use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let slope_map = fs::read_to_string("input.txt")?;
	let slope_map: Vec<String> = slope_map.split('\n').map(String::from).collect();

	let mut row_count: usize = 1; // The first row is handled separately to prevent problems with consuming the first element of an iterator multiple times
	let mut alternate_row_count: usize = 1;
	let mut is_alternate_row = false;

	let mut r1d1_trees: u32 = 0;
	let mut r3d1_trees: u32 = 0;
	let mut r5d1_trees: u32 = 0;
	let mut r7d1_trees: u32 = 0;
	let mut r1d2_trees: u32 = 0;

	let mut tree_lines = slope_map.iter();
	let first_row = tree_lines.next().unwrap();
	let tree_check_char = first_row.chars().next().unwrap();
	if tree_check_char == '#' {
		r1d1_trees += 1;
		r3d1_trees += 1;
		r5d1_trees += 1;
		r7d1_trees += 1;
		r1d2_trees += 1;
	}

	for tree_line in tree_lines {
		if tree_line.is_empty() {
			continue;
		}
		let r1d1 = tree_line.chars().cycle().nth(row_count).unwrap();
		if r1d1 == '#' {
			r1d1_trees += 1;
		}
		let r3d1 = tree_line.chars().cycle().nth(row_count * 3).unwrap();
		if r3d1 == '#' {
			r3d1_trees += 1;
		}
		let r5d1 = tree_line.chars().cycle().nth(row_count * 5).unwrap();
		if r5d1 == '#' {
			r5d1_trees += 1;
		}
		let r7d1 = tree_line.chars().cycle().nth(row_count * 7).unwrap();
		if r7d1 == '#' {
			r7d1_trees += 1;
		}

		if is_alternate_row {
			let r1d2 = tree_line.chars().cycle().nth(alternate_row_count).unwrap();
			if r1d2 == '#' {
				r1d2_trees += 1;
			}
			alternate_row_count += 1;
		}
		is_alternate_row = !is_alternate_row;
		row_count += 1;
	}

	println!("{},{},{},{},{}", r1d1_trees, r3d1_trees, r5d1_trees, r7d1_trees, r1d2_trees);
	println!("{}", r1d1_trees * r3d1_trees * r5d1_trees * r7d1_trees * r1d2_trees);

	Ok(())
}
