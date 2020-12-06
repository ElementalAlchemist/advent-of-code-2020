use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let tree_map = fs::read_to_string("input.txt")?;
	let tree_map: Vec<String> = tree_map.split('\n').map(String::from).collect();

	let mut row_count: usize = 0;
	let mut tree_count: u32 = 0;

	for tree_line in tree_map.iter() {
		if tree_line.is_empty() {
			continue;
		}
		let tree_check_char = tree_line.chars().cycle().nth(row_count * 3).unwrap();
		if tree_check_char == '#' {
			tree_count += 1;
		}
		row_count += 1;
	}

	println!("{}", tree_count);

	Ok(())
}
