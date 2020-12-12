use std::fs;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Coordinates {
	x: i32,
	y: i32,
}

#[derive(Debug)]
enum Direction {
	North,
	East,
	South,
	West,
}

impl Default for Direction {
	fn default() -> Self {
		Self::East
	}
}

impl FromStr for Direction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"N" => Ok(Self::North),
			"E" => Ok(Self::East),
			"S" => Ok(Self::South),
			"W" => Ok(Self::West),
			_ => Err(()),
		}
	}
}

enum DirectionOrForward {
	Direction(Direction),
	Forward,
}

impl FromStr for DirectionOrForward {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s == "F" {
			Ok(Self::Forward)
		} else {
			Ok(Self::Direction(s.parse()?))
		}
	}
}

struct MoveInstruction {
	direction: DirectionOrForward,
	distance: i32,
}

impl MoveInstruction {
	fn process_movement(&self, ship: &mut ShipPositionFacing) {
		match &self.direction {
			DirectionOrForward::Direction(Direction::North) => ship.waypoint_offset.y -= self.distance,
			DirectionOrForward::Direction(Direction::South) => ship.waypoint_offset.y += self.distance,
			DirectionOrForward::Direction(Direction::East) => ship.waypoint_offset.x += self.distance,
			DirectionOrForward::Direction(Direction::West) => ship.waypoint_offset.x -= self.distance,
			DirectionOrForward::Forward => {
				ship.position.x += ship.waypoint_offset.x * self.distance;
				ship.position.y += ship.waypoint_offset.y * self.distance;
			}
		}
	}
}

impl FromStr for MoveInstruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (direction, distance) = s.split_at(1);
		let direction: DirectionOrForward = direction.parse()?;
		let distance: i32 = distance.parse().unwrap();

		Ok(Self { direction, distance })
	}
}

struct TurnInstruction {
	to_right: bool,
	right_angle: bool,
}

impl TurnInstruction {
	fn process_movement(&self, ship: &mut ShipPositionFacing) {
		if self.right_angle {
			if self.to_right {
				Self::rotate_right(ship);
			} else {
				Self::rotate_left(ship);
			}
		} else {
			Self::rotate_back(ship);
		}
	}

	fn rotate_right(ship: &mut ShipPositionFacing) {
		ship.waypoint_offset = Coordinates {
			x: -ship.waypoint_offset.y,
			y: ship.waypoint_offset.x,
		};
	}

	fn rotate_left(ship: &mut ShipPositionFacing) {
		ship.waypoint_offset = Coordinates {
			x: ship.waypoint_offset.y,
			y: -ship.waypoint_offset.x,
		};
	}

	fn rotate_back(ship: &mut ShipPositionFacing) {
		ship.waypoint_offset = Coordinates {
			x: -ship.waypoint_offset.x,
			y: -ship.waypoint_offset.y,
		};
	}
}

impl FromStr for TurnInstruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (direction, turn_amount) = s.split_at(1);
		let mut to_right = direction == "R";
		let right_angle = turn_amount != "180"; // Assuming 90 and 180 degree inputs
		if turn_amount == "270" {
			to_right = !to_right;
		}

		Ok(TurnInstruction { to_right, right_angle })
	}
}

enum Instruction {
	Move(MoveInstruction),
	Turn(TurnInstruction),
}

impl Instruction {
	fn process_movement(&self, ship: &mut ShipPositionFacing) {
		match self {
			Self::Move(instr) => instr.process_movement(ship),
			Self::Turn(instr) => instr.process_movement(ship),
		}
	}
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().next().unwrap() {
			'N' | 'E' | 'W' | 'S' | 'F' => {
				let movement: MoveInstruction = s.parse()?;
				Ok(Self::Move(movement))
			}
			'L' | 'R' => {
				let turn: TurnInstruction = s.parse()?;
				Ok(Self::Turn(turn))
			}
			_ => Err(()),
		}
	}
}

#[derive(Debug)]
struct ShipPositionFacing {
	position: Coordinates,
	waypoint_offset: Coordinates,
}

impl Default for ShipPositionFacing {
	fn default() -> Self {
		let position = Coordinates::default();
		let waypoint_offset = Coordinates { x: 10, y: -1 };

		Self {
			position,
			waypoint_offset,
		}
	}
}

fn main() {
	let instructions = fs::read_to_string("input.txt").expect("Failed to read input file");
	let instructions: Vec<Instruction> = instructions
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(|s| s.parse().unwrap())
		.collect();

	let mut ship = ShipPositionFacing::default();
	for instruction in instructions {
		instruction.process_movement(&mut ship);
	}
	println!("{:?}", ship);
}
