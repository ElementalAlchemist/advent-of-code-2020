use std::fs;
use std::str::FromStr;

struct Range {
	start: u32,
	end: u32,
}

impl Range {
	fn new(start: u32, end: u32) -> Self {
		if start > end {
			Self { start: end, end: start }
		} else {
			Self { start, end }
		}
	}

	fn in_range(&self, num: u32) -> bool {
		num > self.start && num < self.end
	}
}

impl FromStr for Range {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut parts = input.split('-');
		let start = parts.next().unwrap().parse().unwrap();
		let end = parts.next().unwrap().parse().unwrap();
		Ok(Self::new(start, end))
	}
}

struct Rule {
	field_name: String,
	ranges: Vec<Range>,
}

impl Rule {
	fn value_passes_rule(&self, val: u32) -> bool {
		for range in self.ranges.iter() {
			if range.in_range(val) {
				return true;
			}
		}
		false
	}
}

impl FromStr for Rule {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut parts = input.split(": ");
		let field_name = String::from(parts.next().unwrap());
		let ranges = parts.next().unwrap();
		let ranges: Vec<Range> = ranges.split(" or ").map(|s| s.parse().unwrap()).collect();
		Ok(Self { field_name, ranges })
	}
}

struct Ticket {
	values: Vec<u32>,
}

impl FromStr for Ticket {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let values = input.split(',').map(|s| s.parse().unwrap()).collect();
		Ok(Self { values })
	}
}

fn main() {
	let ticket_data = fs::read_to_string("input.txt").expect("Failed to read input file");
	let mut ticket_data_iter = ticket_data.split("\n\n");
	let ticket_rules = ticket_data_iter.next().unwrap();
	let _my_ticket = ticket_data_iter.next().unwrap();
	let other_tickets = ticket_data_iter.next().unwrap();

	let ticket_rules: Vec<Rule> = ticket_rules.split('\n').map(|s| s.parse().unwrap()).collect();
	let other_tickets: Vec<Ticket> = other_tickets
		.split('\n')
		.filter(|s| !s.is_empty())
		.filter(|s| *s != "nearby tickets:")
		.map(|s| s.parse().unwrap())
		.collect();

	let mut invalid_total: u32 = 0;
	for ticket in other_tickets {
		for value in ticket.values {
			let mut is_valid = false;
			for rule in ticket_rules.iter() {
				if rule.value_passes_rule(value) {
					is_valid = true;
					break;
				}
			}
			if !is_valid {
				invalid_total += value;
			}
		}
	}

	println!("{}", invalid_total);
}
