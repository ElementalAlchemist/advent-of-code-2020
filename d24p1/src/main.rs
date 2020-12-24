use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
}

enum TileDirection {
	East,
	Southeast,
	Southwest,
	West,
	Northwest,
	Northeast,
}

impl TileDirection {
	fn next_coord(&self, coord: Coordinate) -> Coordinate {
		match self {
			Self::East => Coordinate {
				x: coord.x + 1,
				y: coord.y,
			},
			Self::Southeast => Coordinate {
				x: coord.x,
				y: coord.y + 1,
			},
			Self::Southwest => Coordinate {
				x: coord.x - 1,
				y: coord.y + 1,
			},
			Self::West => Coordinate {
				x: coord.x - 1,
				y: coord.y,
			},
			Self::Northwest => Coordinate {
				x: coord.x,
				y: coord.y - 1,
			},
			Self::Northeast => Coordinate {
				x: coord.x + 1,
				y: coord.y - 1,
			},
		}
	}
}

impl FromStr for TileDirection {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"e" => Ok(Self::East),
			"se" => Ok(Self::Southeast),
			"sw" => Ok(Self::Southwest),
			"w" => Ok(Self::West),
			"nw" => Ok(Self::Northwest),
			"ne" => Ok(Self::Northeast),
			_ => Err(()),
		}
	}
}

struct TileDirectionList {
	directions: Vec<TileDirection>,
}

impl TileDirectionList {
	fn destination(&self) -> Coordinate {
		let mut coord = Coordinate { x: 0, y: 0 };
		for dir in self.directions.iter() {
			coord = dir.next_coord(coord);
		}
		coord
	}
}

impl FromStr for TileDirectionList {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut next_direction = String::new();
		let mut directions: Vec<TileDirection> = Vec::new();
		for char in input.chars() {
			next_direction.push(char);
			if next_direction != "n" && next_direction != "s" {
				directions.push(next_direction.parse().unwrap_or_else(|_| panic!("Failed to parse value {}", next_direction)));
				next_direction.clear();
			}
		}
		Ok(Self { directions })
	}
}

fn main() {
	let tile_directions: Vec<TileDirectionList> = {
		let input = fs::read_to_string("input.txt").expect("Failed to read input file");
		input
			.split('\n')
			.filter(|s| !s.is_empty())
			.map(|s| s.parse().unwrap())
			.collect()
	};

	let mut black_tiles: HashSet<Coordinate> = HashSet::new();
	for tile in tile_directions.iter() {
		let tile_coord = tile.destination();
		if black_tiles.contains(&tile_coord) {
			black_tiles.remove(&tile_coord);
		} else {
			black_tiles.insert(tile_coord);
		}
	}

	println!("{}", black_tiles.len());
}
