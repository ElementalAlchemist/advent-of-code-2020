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

	fn data_without_borders(&self) -> [[bool; 8]; 8] {
		let mut borderless_tile = [[false; 8]; 8];

		for (row, row_data) in self.tile_data[1..9].iter().enumerate() {
			for (col, col_data) in row_data[1..9].iter().enumerate() {
				if *col_data {
					borderless_tile[row][col] = true;
				}
			}
		}

		borderless_tile
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

fn transform_full_image(image: &[bool], pixel_grid_size: usize, orientation: Orientation) -> Vec<bool> {
	let mut new_image: Vec<bool> = vec![false; image.len()];

	for (pixel_index, pixel) in image.iter().enumerate() {
		let new_coord = match orientation {
			Orientation::Original => pixel_index,
			Orientation::Rotated90 => {
				let x = pixel_index % pixel_grid_size;
				let y = pixel_index / pixel_grid_size;
				// y * pixel_grid_size + x
				// x = y
				// y = pixel_grid_size - x - 1
				x * pixel_grid_size + (pixel_grid_size - y - 1)
			}
			Orientation::Rotated180 => image.len() - pixel_index - 1,
			Orientation::Rotated270 => {
				let x = pixel_index % pixel_grid_size;
				let y = pixel_index / pixel_grid_size;
				(pixel_grid_size - x - 1) * pixel_grid_size + y
			}
			Orientation::FlippedHorizontally => {
				let x = pixel_index % pixel_grid_size;
				let y = pixel_index / pixel_grid_size;
				y * pixel_grid_size + (pixel_grid_size - x - 1)
			}
			Orientation::FlippedVertically => {
				let x = pixel_index % pixel_grid_size;
				let y = pixel_index / pixel_grid_size;
				(pixel_grid_size - y - 1) * pixel_grid_size + x
			}
			Orientation::FlippedHorizontallyRotated => {
				let x = pixel_index % pixel_grid_size;
				let y = pixel_index / pixel_grid_size;
				let x = pixel_grid_size - x - 1;
				x * pixel_grid_size + (pixel_grid_size - y - 1)
			}
			Orientation::FlippedVerticallyRotated => {
				let x = pixel_index % pixel_grid_size;
				let y = pixel_index / pixel_grid_size;
				let y = pixel_grid_size - y - 1;
				x * pixel_grid_size + (pixel_grid_size - y - 1)
			}
		};
		new_image[new_coord] = *pixel;
	}

	new_image
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

	let pixel_grid_size = grid_size * 8;
	let mut full_image: Vec<bool> = Vec::with_capacity(tiles.len() * 64);
	for _ in 0..tiles.len() {
		full_image.extend([false; 64].iter());
	}
	for (tile_index, (tile, orientation)) in tile_grid.iter().enumerate() {
		let tile = orientation.transform_tile(tile);
		let tile_data = tile.data_without_borders();
		for (pixel_index, pixel) in tile_data.iter().flatten().enumerate() {
			full_image[((tile_index / grid_size) * 8 * pixel_grid_size)
				+ ((tile_index % grid_size) * 8)
				+ ((pixel_index / 8) * pixel_grid_size)
				+ (pixel_index % 8)] = *pixel;
		}
	}

	let sea_monster = [
		[
			false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
			false, false, false, true, false,
		],
		[
			true, false, false, false, false, true, true, false, false, false, false, true, true, false, false, false,
			false, true, true, true,
		],
		[
			false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false,
			true, false, false, false,
		],
	];
	let sea_monster: Vec<bool> = sea_monster
		.iter()
		.enumerate()
		.map(|(index, v)| {
			if index == 2 {
				v.iter().copied().collect()
			} else {
				let mut val: Vec<bool> = v.iter().copied().collect();
				for _ in val.len()..pixel_grid_size {
					val.push(false);
				}
				val
			}
		})
		.flatten()
		.collect();

	let mut sea_monster_waves: usize = 0;
	for orientation in Orientation::values() {
		let full_image = transform_full_image(&full_image, pixel_grid_size, orientation);
		for (pixel_index, _) in full_image.iter().enumerate().step_by(pixel_grid_size) {
			if full_image.len() - pixel_index < pixel_grid_size * 3 {
				continue; // Can't do these rows because we'll pass the end
			}
			let row_data = &full_image[pixel_index..pixel_index + pixel_grid_size * 3];
			let max_check_start = row_data.len() - sea_monster.len();
			for pixel_start in 0..max_check_start {
				let mut found_monster = true;
				for (pixel_value, sea_monster_value) in row_data.iter().skip(pixel_start).zip(sea_monster.iter()) {
					if *sea_monster_value && !*pixel_value {
						found_monster = false;
						break;
					}
				}
				if found_monster {
					sea_monster_waves += 15;
				}
			}
		}
		if sea_monster_waves > 0 {
			break;
		}
	}

	let pixel_count = full_image.iter().filter(|x| **x).count();
	println!(
		"{} - {} = {}",
		pixel_count,
		sea_monster_waves,
		pixel_count - sea_monster_waves
	);
}
