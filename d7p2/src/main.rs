use std::collections::{HashMap, VecDeque};
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
	let bag_rules: HashMap<String, Vec<(String, u32)>> = bag_rules
		.iter()
		.map(|rule| (rule.outer_bag.clone(), rule.inner_bags.clone()))
		.collect();

	let mut gold_contains: Vec<(String, u32)> = Vec::new();
	let mut add_bags: VecDeque<(String, u32)> = VecDeque::new();
	for bag in bag_rules.get("shiny gold").unwrap().iter() {
		add_bags.push_back(bag.clone());
	}

	while !add_bags.is_empty() {
		let next_bag = add_bags.pop_front().unwrap();
		for more_bag in bag_rules.get(&next_bag.0).unwrap().iter() {
			add_bags.push_back((more_bag.0.clone(), more_bag.1 * next_bag.1));
		}
		gold_contains.push(next_bag);
	}

	let mut total_bags: u32 = 0;
	for contained_bags in gold_contains.iter() {
		total_bags += contained_bags.1;
	}
	println!("{}", total_bags);

	Ok(())
}
