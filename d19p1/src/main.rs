use std::fs;
use std::str::FromStr;

enum RuleType<'a> {
	Value(&'a str),
	CheckOtherRules(Vec<Vec<usize>>),
}

struct RuleList<'a> {
	rules: Vec<RuleType<'a>>,
}

impl<'a> RuleList<'a> {
	fn new() -> Self {
		Self { rules: Vec::new() }
	}

	fn check(&self, rule: usize, val: &str) -> bool {
		let (valid, remainder) = self.check_inner(rule, val);
		valid && remainder.is_empty()
	}

	fn check_inner(&self, rule: usize, val: &'a str) -> (bool, &str) {
		let rule = &self.rules[rule];
		match rule {
			RuleType::Value(value) => {
				let mut str_iter = val.chars();
				for ch in value.chars() {
					if let Some(c) = str_iter.next() {
						if c != ch {
							return (false, *value);
						}
					} else {
						return (false, *value);
					}
				}
				let remainder = &val[val.len() - str_iter.count()..];
				(true, remainder)
			}
			RuleType::CheckOtherRules(rules_list) => {
				'rule_list: for rule_list in rules_list.iter() {
					let mut val = val;
					for rule in rule_list.iter() {
						let data = self.check_inner(*rule, val);
						val = data.1;
						if !data.0 {
							continue 'rule_list;
						}
					}
					return (true, val);
				}
				(false, val)
			}
		}
	}
}

enum RuleDataValue {
	Number(usize),
	Or,
}

impl FromStr for RuleDataValue {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		if input == "|" {
			Ok(Self::Or)
		} else {
			Ok(Self::Number(input.parse().unwrap()))
		}
	}
}

fn main() {
	let message_data_rules = fs::read_to_string("input.txt").expect("Failed to read input file");
	let mut message_data_rules_iter = message_data_rules.split("\n\n");
	let rules_spec: Vec<&str> = message_data_rules_iter.next().unwrap().split('\n').collect();
	let messages: Vec<&str> = message_data_rules_iter
		.next()
		.unwrap()
		.split('\n')
		.filter(|s| !s.is_empty())
		.collect();

	let mut rules = RuleList::new();
	for rule in rules_spec {
		let mut rule_parts = rule.split(": ");
		let rule_id: usize = rule_parts.next().unwrap().parse().unwrap();
		let rule_data = rule_parts.next().unwrap();
		let rule_val = if rule_data.starts_with('"') {
			let rule_data = &rule_data[1..rule_data.len() - 1];
			RuleType::Value(rule_data)
		} else {
			let other_rules_data: Vec<RuleDataValue> = rule_data.split(' ').map(|s| s.parse().unwrap()).collect();
			let mut rules_buffer: Vec<usize> = Vec::new();
			let mut other_rules: Vec<Vec<usize>> = Vec::new();
			for data in other_rules_data.iter() {
				match data {
					RuleDataValue::Number(num) => rules_buffer.push(*num),
					RuleDataValue::Or => other_rules.push(rules_buffer.drain(..).collect()),
				}
			}
			other_rules.push(rules_buffer);
			RuleType::CheckOtherRules(other_rules)
		};
		while rules.rules.len() <= rule_id {
			rules.rules.push(RuleType::Value(""));
		}
		rules.rules[rule_id] = rule_val;
	}

	let mut match_count: usize = 0;
	for message in messages.iter() {
		if rules.check(0, *message) {
			match_count += 1;
		}
	}
	println!("{}", match_count);
}
