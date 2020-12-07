use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

struct BagContentsRule {
	outer_bag: String,
	inner_bags: Vec<(String, u32)>,
}

impl BagContentsRule {
	fn new(spec: &String) -> Self {
		let mut bag_data = spec.split(" bags contain ");
		let outer_bag = bag_data.next().expect("Failed to parse bag data");
		let inner_bags_str = bag_data.next().expect("Failed to parse bag data");
		let outer_bag = String::from(outer_bag);
		if inner_bags_str == "no other bags" {
			return Self {
				outer_bag,
				inner_bags: Vec::new(),
			};
		}

		let inner_bags_str: Vec<&str> = inner_bags_str.split(", ").collect();
		let mut inner_bags: Vec<(String, u32)> = Vec::with_capacity(inner_bags_str.len());

		for bag_desc in inner_bags_str.iter() {
			let space_pos = bag_desc.find(' ').expect("Failed to parse bag description");
			let (count, color) = bag_desc.split_at(space_pos);
			let count: u32 = count
				.parse()
				.unwrap_or_else(|_| panic!("Failed to parse bag count: {}", count));
			let color = if let Some(color) = color.strip_suffix(" bags") {
				color
			} else if let Some(color) = color.strip_suffix(" bag") {
				color
			} else {
				panic!("Failed to parse bag color");
			};
			let color = if let Some(c) = color.strip_prefix(' ') {
				c
			} else {
				color
			};
			inner_bags.push((String::from(color), count));
		}

		Self { outer_bag, inner_bags }
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let bag_rules = fs::read_to_string("input.txt")?;
	let bag_rules: Vec<String> = bag_rules
		.split(".\n")
		.filter(|s| !s.is_empty())
		.map(String::from)
		.collect();

	let bag_rules: Vec<BagContentsRule> = bag_rules.iter().map(BagContentsRule::new).collect();
	let mut contained_by: HashMap<String, HashSet<String>> = HashMap::new();

	for rule in bag_rules.iter() {
		for bag in rule.inner_bags.iter() {
			let bag_entry = contained_by.entry(bag.0.clone()).or_default();
			bag_entry.insert(rule.outer_bag.clone());
		}
	}

	let mut can_contain_gold: HashSet<String> = HashSet::new();
	for bag in contained_by.entry(String::from("shiny gold")).or_default().iter() {
		can_contain_gold.insert(bag.clone());
	}
	loop {
		let mut add_bags: Vec<String> = Vec::new();
		for check_bag in can_contain_gold.iter() {
			for containing_bag in contained_by.entry(check_bag.clone()).or_default().iter() {
				if !can_contain_gold.contains(containing_bag) {
					add_bags.push(containing_bag.clone());
				}
			}
		}
		if add_bags.is_empty() {
			break;
		}
		for bag in add_bags {
			can_contain_gold.insert(bag);
		}
	}

	println!("{}", can_contain_gold.len());

	Ok(())
}
