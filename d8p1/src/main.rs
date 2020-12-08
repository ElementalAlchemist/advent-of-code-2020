use std::collections::HashSet;
use std::fs;

enum InstructionType {
	Accumulate,
	Jump,
	NoOp,
}

impl InstructionType {
	fn from_str(instruction: &str) -> Self {
		if instruction == "acc" {
			Self::Accumulate
		} else if instruction == "jmp" {
			Self::Jump
		} else if instruction == "nop" {
			Self::NoOp
		} else {
			panic!("Instruction of wrong type found: {}", instruction);
		}
	}
}

struct Instruction {
	inst_type: InstructionType,
	val: i32,
}

impl Instruction {
	fn from_instruction_str(instruction: &str) -> Self {
		let mut vals = instruction.split(' ');
		let inst_type = vals.next().unwrap();
		let val = vals.next().unwrap();
		assert!(vals.next().is_none());

		let inst_type = InstructionType::from_str(inst_type);
		let val: i32 = val.parse().unwrap();
		Self { inst_type, val }
	}
}

fn main() {
	let instructions = fs::read_to_string("input.txt").expect("Failed to read input file");
	let instructions: Vec<String> = instructions
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(String::from)
		.collect();
	let instructions: Vec<Instruction> = instructions
		.iter()
		.map(|instruction| Instruction::from_instruction_str(instruction))
		.collect();

	let mut visited_nodes: HashSet<usize> = HashSet::new();
	let mut current_node: usize = 0;
	let mut accumulator_value: i32 = 0;
	loop {
		visited_nodes.insert(current_node);
		let current_instruction = &instructions[current_node];
		match current_instruction.inst_type {
			InstructionType::Accumulate => {
				accumulator_value += current_instruction.val;
				current_node += 1;
			}
			InstructionType::Jump => {
				let val = current_instruction.val.abs() as usize;
				if current_instruction.val > 0 {
					current_node += val;
				} else {
					current_node -= val;
				}
			}
			InstructionType::NoOp => {
				current_node += 1;
			}
		}
		if visited_nodes.contains(&current_node) {
			break;
		}
	}
	println!("{}", accumulator_value);
}
