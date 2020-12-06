use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let input_file = fs::read_to_string("input.txt")?;
	let inputs: Vec<u32> = input_file.split('\n').map(|x| x.parse().unwrap()).collect();

	'add_loop: for first_num in inputs.iter() {
		for second_num in inputs.iter() {
			if first_num + second_num == 2020 {
				println!("{}", first_num * second_num);
				break 'add_loop;
			}
		}
	}

	Ok(())
}
