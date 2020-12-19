use std::fs;
use std::str::FromStr;

enum RuleType {
	Value(char),
	CheckOtherRules(Vec<Vec<usize>>),
}

struct RuleList {
	rules: Vec<RuleType>,
}

impl RuleList {
	fn new() -> Self {
		Self { rules: Vec::new() }
	}

	fn check(&self, rule: usize, val: &str) -> bool {
		let mut pointers: Vec<Vec<usize>> = vec![vec![rule]];

		for check_char in val.chars() {
			if pointers.is_empty() {
				return false;
			}
			pointers = self.advance_pointer(check_char, pointers);
		}
		pointers.iter().any(|x| x.is_empty())
	}

	fn advance_pointer(&self, next_character: char, mut ptr: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
		let mut new_ptrs: Vec<Vec<usize>> = Vec::new();
		while !ptr.is_empty() {
			let mut rule_list = ptr.remove(0);
			if rule_list.is_empty() {
				// We completed the rules last character, but there's more string, so we must disregard this rule
				continue;
			}
			let next_rule = rule_list.remove(0);
			match &self.rules[next_rule] {
				RuleType::Value(match_char) => {
					if next_character == *match_char {
						new_ptrs.push(rule_list.clone());
					}
				}
				RuleType::CheckOtherRules(other_rule_ptr) => {
					for other_rule_list in other_rule_ptr {
						ptr.push(
							vec![other_rule_list.clone(), rule_list.clone()]
								.iter()
								.flatten()
								.copied()
								.collect(),
						);
					}
				}
			}
		}
		new_ptrs
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
			let rule_data = rule_data.chars().nth(1).unwrap(); // All string rules are single characters
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
			rules.rules.push(RuleType::Value(' '));
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
