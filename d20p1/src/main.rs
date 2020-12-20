use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Clone)]
struct Tile {
	id: u32,
	tile_data: [[bool; 10]; 10],
}

impl Tile {
	fn rotate_90(&self) -> Self {
		let mut new_tile_data = [[false; 10]; 10];
		for (row, row_data) in self.tile_data.iter().enumerate() {
			for (col, col_data) in row_data.iter().enumerate() {
				if *col_data {
					new_tile_data[col][9 - row] = true;
				}
			}
		}
		Self {
			id: self.id,
			tile_data: new_tile_data,
		}
	}

	fn rotate_180(&self) -> Self {
		let mut new_tile_data = [[false; 10]; 10];
		for (row, row_data) in self.tile_data.iter().enumerate() {
			for (col, col_data) in row_data.iter().enumerate() {
				if *col_data {
					new_tile_data[9 - col][9 - row] = true;
				}
			}
		}
		Self {
			id: self.id,
			tile_data: new_tile_data,
		}
	}

	fn rotate_270(&self) -> Self {
		let mut new_tile_data = [[false; 10]; 10];
		for (row, row_data) in self.tile_data.iter().enumerate() {
			for (col, col_data) in row_data.iter().enumerate() {
				if *col_data {
					new_tile_data[9 - col][row] = true;
				}
			}
		}
		Self {
			id: self.id,
			tile_data: new_tile_data,
		}
	}

	fn flip_horizontal(&self) -> Self {
		let mut new_tile_data = [[false; 10]; 10];
		for (row, row_data) in self.tile_data.iter().enumerate() {
			for (col, col_data) in row_data.iter().enumerate() {
				if *col_data {
					new_tile_data[row][9 - col] = true;
				}
			}
		}
		Self {
			id: self.id,
			tile_data: new_tile_data,
		}
	}

	fn flip_vertical(&self) -> Self {
		let mut new_tile_data = [[false; 10]; 10];
		for (row, row_data) in self.tile_data.iter().enumerate() {
			for (col, col_data) in row_data.iter().enumerate() {
				if *col_data {
					new_tile_data[9 - row][col] = true;
				}
			}
		}
		Self {
			id: self.id,
			tile_data: new_tile_data,
		}
	}

	fn top_side(&self) -> [bool; 10] {
		let mut side_data = [false; 10];
		for (col, data) in self.tile_data[0].iter().enumerate() {
			if *data {
				side_data[col] = true;
			}
		}
		side_data
	}

	fn right_side(&self) -> [bool; 10] {
		let mut side_data = [false; 10];
		for (row, data) in self.tile_data.iter().enumerate() {
			if data[9] {
				side_data[row] = true;
			}
		}
		side_data
	}

	fn bottom_side(&self) -> [bool; 10] {
		let mut side_data = [false; 10];
		for (col, data) in self.tile_data[9].iter().enumerate() {
			if *data {
				side_data[col] = true;
			}
		}
		side_data
	}

	fn left_side(&self) -> [bool; 10] {
		let mut side_data = [false; 10];
		for (row, data) in self.tile_data.iter().enumerate() {
			if data[0] {
				side_data[row] = true;
			}
		}
		side_data
	}
}

impl PartialEq for Tile {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Tile {}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Side {
	Top,
	Right,
	Bottom,
	Left,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Orientation {
	Original,
	Rotated90,
	Rotated180,
	Rotated270,
	FlippedHorizontally,
	FlippedVertically,
	FlippedHorizontallyRotated,
	FlippedVerticallyRotated,
}

impl Orientation {
	fn values() -> Vec<Self> {
		vec![
			Orientation::Original,
			Orientation::Rotated90,
			Orientation::Rotated180,
			Orientation::Rotated270,
			Orientation::FlippedHorizontally,
			Orientation::FlippedVertically,
			Orientation::FlippedHorizontallyRotated,
			Orientation::FlippedVerticallyRotated,
		]
	}

	fn transform_tile(&self, tile: &Tile) -> Tile {
		match self {
			Orientation::Original => tile.clone(),
			Orientation::Rotated90 => tile.rotate_90(),
			Orientation::Rotated180 => tile.rotate_180(),
			Orientation::Rotated270 => tile.rotate_270(),
			Orientation::FlippedHorizontally => tile.flip_horizontal(),
			Orientation::FlippedVertically => tile.flip_vertical(),
			Orientation::FlippedHorizontallyRotated => tile.flip_horizontal().rotate_90(),
			Orientation::FlippedVerticallyRotated => tile.flip_vertical().rotate_90(),
		}
	}
}

struct TileMatch {
	side: Side,
	orientation: Orientation,
	with_tile: u32,
	other_orientation: Orientation,
}

impl FromStr for Tile {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut lines = input.split('\n').filter(|s| !s.is_empty());
		let id = lines.next().unwrap();
		let id = id.strip_prefix("Tile ").unwrap();
		let id = id.strip_suffix(':').unwrap();
		let id = id.parse().unwrap();

		let mut tile_data = [[false; 10]; 10];
		let tile_input_data: Vec<&str> = lines.collect();
		if tile_input_data.len() != 10 {
			return Err(());
		}
		for (tile_line, tile_line_data) in tile_input_data.iter().enumerate() {
			for (tile_pos, tile_pixel_data) in tile_line_data.chars().enumerate() {
				if tile_pixel_data == '#' {
					tile_data[tile_line][tile_pos] = true;
				}
			}
		}

		Ok(Self { id, tile_data })
	}
}

fn get_all_matches(tile: &Tile, other_tile: &Tile) -> Vec<TileMatch> {
	let mut matches: Vec<TileMatch> = Vec::new();
	for orientation in Orientation::values() {
		let tile = orientation.transform_tile(tile);
		for other_orientation in Orientation::values() {
			let other_tile = other_orientation.transform_tile(other_tile);
			matches.extend(get_side_matches(&tile, &other_tile, orientation, other_orientation));
		}
	}
	matches
}

fn get_side_matches(
	tile: &Tile,
	other_tile: &Tile,
	orientation: Orientation,
	other_orientation: Orientation,
) -> Vec<TileMatch> {
	let mut matches: Vec<TileMatch> = Vec::new();

	if tile.top_side() == other_tile.bottom_side() {
		matches.push(TileMatch {
			side: Side::Top,
			orientation,
			with_tile: other_tile.id,
			other_orientation,
		});
	}
	if tile.right_side() == other_tile.left_side() {
		matches.push(TileMatch {
			side: Side::Right,
			orientation,
			with_tile: other_tile.id,
			other_orientation,
		});
	}
	if tile.bottom_side() == other_tile.top_side() {
		matches.push(TileMatch {
			side: Side::Bottom,
			orientation,
			with_tile: other_tile.id,
			other_orientation,
		});
	}
	if tile.left_side() == other_tile.right_side() {
		matches.push(TileMatch {
			side: Side::Left,
			orientation,
			with_tile: other_tile.id,
			other_orientation,
		});
	}
	matches
}

fn add_tile<'a>(
	tiles: &'a HashMap<u32, Tile>,
	grid_size: usize,
	tile_matches: &HashMap<u32, Vec<TileMatch>>,
	tile_grid: &Vec<(&'a Tile, Orientation)>,
) -> Option<Vec<(&'a Tile, Orientation)>> {
	let (check_tile, check_side) = if tile_grid.len() % grid_size == 0 {
		(&tile_grid[tile_grid.len() - grid_size], Side::Bottom)
	} else {
		(tile_grid.last().unwrap(), Side::Right)
	};
	for possible_matching_tile in tile_matches[&check_tile.0.id].iter() {
		if possible_matching_tile.orientation != check_tile.1 || possible_matching_tile.side != check_side {
			continue;
		}
		if tile_grid
			.iter()
			.any(|entry| entry.0.id == possible_matching_tile.with_tile)
		{
			continue;
		}

		let mut grid = tile_grid.clone();
		grid.push((
			&tiles[&possible_matching_tile.with_tile],
			possible_matching_tile.other_orientation,
		));
		if grid.len() == tiles.len() {
			return Some(grid);
		}
		if let Some(new_grid) = add_tile(tiles, grid_size, tile_matches, &grid) {
			return Some(new_grid);
		}
	}
	None
}

fn main() {
	let tiles = fs::read_to_string("input.txt").expect("Failed to read input file");
	let tiles: HashMap<u32, Tile> = tiles
		.split("\n\n")
		.map(|s| {
			let tile: Tile = s.parse().unwrap();
			(tile.id, tile)
		})
		.collect();

	let mut tile_matches: HashMap<u32, Vec<TileMatch>> = HashMap::new();

	for tile in tiles.values() {
		let mut matches: Vec<TileMatch> = Vec::new();

		for other_tile in tiles.values() {
			if tile.id == other_tile.id {
				continue; // Don't attempt to match this tile with itself
			}
			matches.extend(get_all_matches(tile, other_tile));
		}
		if !matches.is_empty() {
			tile_matches.insert(tile.id, matches);
		}
	}

	let grid_size = (tiles.len() as f64).sqrt() as usize;
	let mut tile_grid: Vec<(&Tile, Orientation)> = Vec::new();
	'tile_loop: for tile in tiles.values() {
		for orientation in Orientation::values() {
			tile_grid.push((tile, orientation));
			if let Some(grid) = add_tile(&tiles, grid_size, &tile_matches, &tile_grid) {
				tile_grid = grid;
				break 'tile_loop;
			}
			tile_grid.clear();
		}
	}

	if tile_grid.is_empty() {
		eprintln!("ERROR: No grid found!");
	} else {
		let top_left_corner = tile_grid[0].0.id as u64;
		let top_right_corner = tile_grid[grid_size - 1].0.id as u64;
		let bottom_left_corner = tile_grid[tile_grid.len() - grid_size].0.id as u64;
		let bottom_right_corner = tile_grid[tile_grid.len() - 1].0.id as u64;
		println!(
			"{}",
			top_left_corner * top_right_corner * bottom_left_corner * bottom_right_corner
		);
	}
}
