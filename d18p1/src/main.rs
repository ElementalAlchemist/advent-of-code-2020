use std::fs;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Operation {
	Addition,
	Multiplication,
}

impl Operation {
	fn apply(&self, lhs: u64, rhs: u64) -> u64 {
		match self {
			Self::Addition => lhs + rhs,
			Self::Multiplication => lhs * rhs,
		}
	}
}

enum ExpressionToken {
	Number(u64),
	Operator(Operation),
	Group(Expression),
}

struct Expression {
	tokens: Vec<ExpressionToken>,
}

impl Expression {
	fn calculate(&self) -> u64 {
		let mut answer_so_far: u64 = 0;
		let mut current_operation: Operation = Operation::Addition;
		for token in self.tokens.iter() {
			match token {
				ExpressionToken::Number(num) => {
					answer_so_far = current_operation.apply(answer_so_far, *num);
				}
				ExpressionToken::Operator(op) => current_operation = *op,
				ExpressionToken::Group(expr) => {
					answer_so_far = current_operation.apply(answer_so_far, expr.calculate())
				}
			}
		}
		answer_so_far
	}
}

impl FromStr for Expression {
	type Err = ();

	/// A simple expression parser
	/// Assumes expressions have space-separated operations and numbers
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut tokens: Vec<ExpressionToken> = Vec::new();
		let mut token_group_stack: Vec<Vec<ExpressionToken>> = Vec::new();
		for mut token in input.split(' ') {
			while let Some(stripped_token) = token.strip_prefix('(') {
				token_group_stack.push(Vec::new());
				token = stripped_token;
			}
			let mut end_group_count: usize = 0;
			while let Some(stripped_token) = token.strip_suffix(')') {
				end_group_count += 1;
				token = stripped_token;
			}

			let next_token = if token == "*" {
				ExpressionToken::Operator(Operation::Multiplication)
			} else if token == "+" {
				ExpressionToken::Operator(Operation::Addition)
			} else {
				ExpressionToken::Number(token.parse().unwrap())
			};
			if token_group_stack.is_empty() {
				tokens.push(next_token);
			} else {
				token_group_stack.last_mut().unwrap().push(next_token);
			}

			for _ in 0..end_group_count {
				let finished_group = token_group_stack.pop().unwrap();
				if token_group_stack.is_empty() {
					tokens.push(ExpressionToken::Group(Expression { tokens: finished_group }));
				} else {
					token_group_stack
						.last_mut()
						.unwrap()
						.push(ExpressionToken::Group(Expression { tokens: finished_group }));
				}
			}
		}

		Ok(Self { tokens })
	}
}

fn main() {
	let expressions = fs::read_to_string("input.txt").expect("Failed to read input file");
	let expressions: Vec<Expression> = expressions
		.split('\n')
		.filter(|s| !s.is_empty())
		.map(|s| s.parse().unwrap())
		.collect();

	let mut sum: u64 = 0;
	for expr in expressions.iter() {
		sum += expr.calculate();
	}

	println!("{}", sum);
}
