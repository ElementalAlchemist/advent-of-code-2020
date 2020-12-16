use std::collections::{HashMap, HashSet};
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
		num >= self.start && num <= self.end
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
	let my_ticket = ticket_data_iter.next().unwrap();
	let other_tickets = ticket_data_iter.next().unwrap();

	let mut ticket_rules: HashMap<String, Rule> = ticket_rules
		.split('\n')
		.map(|s| s.parse().unwrap())
		.map(|rule: Rule| (String::from(rule.field_name.clone()), rule))
		.collect();
	let my_ticket: Ticket = my_ticket.split('\n').skip(1).next().unwrap().parse().unwrap();
	let mut other_tickets: Vec<Ticket> = other_tickets
		.split('\n')
		.filter(|s| !s.is_empty())
		.filter(|s| *s != "nearby tickets:")
		.map(|s| s.parse().unwrap())
		.collect();

	let mut invalid_ticket_indices: Vec<usize> = Vec::new();
	for (ticket_index, ticket) in other_tickets.iter().enumerate() {
		for value in ticket.values.iter() {
			let mut is_valid = false;
			for rule in ticket_rules.values() {
				if rule.value_passes_rule(*value) {
					is_valid = true;
					break;
				}
			}
			if !is_valid {
				invalid_ticket_indices.push(ticket_index);
			}
		}
	}
	for index in invalid_ticket_indices.iter().rev() {
		other_tickets.remove(*index);
	}

	let mut ticket_fields: Vec<String> = Vec::with_capacity(my_ticket.values.len());
	for _ in my_ticket.values.iter() {
		ticket_fields.push(String::from(""));
	}
	while ticket_fields.contains(&String::from("")) {
		for (field_index, _) in my_ticket.values.iter().enumerate() {
			let mut possible_fields: HashSet<String> = ticket_rules.keys().map(String::from).collect();
			for ticket in other_tickets.iter() {
				let mut impossible_fields: Vec<String> = Vec::new();
				for field in possible_fields.iter() {
					if !ticket_rules[field].value_passes_rule(ticket.values[field_index]) {
						impossible_fields.push(field.clone());
					}
				}
				for field in impossible_fields.iter() {
					possible_fields.remove(field);
				}
			}
			if possible_fields.len() == 1 {
				let field = possible_fields.iter().next().unwrap();
				ticket_rules.remove(field);
				ticket_fields[field_index] = field.clone();
			}
		}
	}

	let mut total_value: u64 = 1;
	for (field_index, field_name) in ticket_fields.iter().enumerate() {
		if field_name.starts_with("departure") {
			total_value *= my_ticket.values[field_index] as u64;
		}
	}
	println!("{}", total_value);
}
