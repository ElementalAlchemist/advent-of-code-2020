use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

struct Mask {
	mask: Vec<Box<dyn Fn(Vec<u64>) -> Vec<u64>>>,
}

impl Mask {
	fn apply(&self, input: u64) -> Vec<u64> {
		let mut result = vec![input];
		for mask_fn in self.mask.iter() {
			result = mask_fn(result);
		}
		result
	}
}

impl Default for Mask {
	fn default() -> Self {
		Self { mask: Vec::new() }
	}
}

impl FromStr for Mask {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut chars: Vec<char> = Vec::with_capacity(36);
		for input_char in input.chars().rev() {
			if input_char == '0' || input_char == '1' || input_char == 'X' {
				chars.push(input_char);
			} else {
				return Err(());
			}
		}

		let mut base: u64 = 1;
		let mut mask: Vec<Box<dyn Fn(Vec<u64>) -> Vec<u64>>> = Vec::new();
		for mask_bit in chars {
			match mask_bit {
				'0' => {}
				'1' => {
					let mask_oper = base;
					mask.push(Box::new(move |input| {
						let mut output: Vec<u64> = Vec::with_capacity(input.len());
						for val in input.iter() {
							output.push(*val | mask_oper);
						}
						output
					}));
				}
				_ => {
					let mask_or_oper = base;
					let mask_and_oper = u64::MAX - base;
					mask.push(Box::new(move |input| {
						let mut output: Vec<u64> = Vec::new();
						for val in input.iter() {
							output.push(*val | mask_or_oper);
							output.push(*val & mask_and_oper);
						}
						output
					}));
				}
			}
			base *= 2;
		}

		Ok(Self { mask })
	}
}

struct MemorySetInstruction {
	address: u64,
	value: u64,
}

enum Instruction {
	Mask(Mask),
	Set(MemorySetInstruction),
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut parts = input.split(" = ");
		let command = parts.next().unwrap();
		let data = parts.next().unwrap();
		if let Some(_) = parts.next() {
			return Err(());
		}

		if command == "mask" {
			return Ok(Self::Mask(data.parse()?));
		}
		if !command.starts_with("mem") {
			return Err(());
		}
		let (_, address) = command.split_at(command.find('[').unwrap() + 1);
		let address = address.strip_suffix(']').unwrap();
		let address: u64 = address.parse().unwrap();
		Ok(Self::Set(MemorySetInstruction {
			address,
			value: data.parse().unwrap(),
		}))
	}
}

fn main() {
	let memory_input = fs::read_to_string("input.txt").expect("Failed to read input file");
	let memory_input: Vec<Instruction> = memory_input
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(|s| s.parse().unwrap())
		.collect();

	let mut memory: HashMap<u64, u64> = HashMap::new();
	let mut mask: Mask = Mask::default();
	for instr in memory_input {
		match instr {
			Instruction::Mask(new_mask) => mask = new_mask,
			Instruction::Set(MemorySetInstruction { address, value }) => {
				let memory_addrs = mask.apply(address);
				for addr in memory_addrs {
					let addr = memory.entry(addr).or_insert(0);
					*addr = value;
				}
			}
		}
	}

	let sum: u64 = memory.values().sum();
	println!("{}", sum);
}
