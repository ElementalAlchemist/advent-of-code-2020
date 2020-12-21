use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

struct IngredientsLabel {
	ingredients: Vec<String>,
	allergens: Vec<String>,
}

impl FromStr for IngredientsLabel {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut ingredients_and_allergens = input.split(" (contains ");
		let ingredients = ingredients_and_allergens.next().unwrap();
		let allergens = if let Some(allergens) = ingredients_and_allergens.next() {
			allergens.strip_suffix(')').unwrap_or("")
		} else {
			""
		};

		let ingredients: Vec<String> = ingredients.split(' ').map(String::from).collect();
		let allergens: Vec<String> = allergens.split(", ").map(String::from).collect();
		Ok(Self { ingredients, allergens })
	}
}

fn main() {
	let ingredients_lists: Vec<IngredientsLabel> = {
		let input = fs::read_to_string("input.txt").expect("Failed to read input file");
		input
			.split('\n')
			.filter(|s| !s.is_empty())
			.map(|s| s.parse().unwrap())
			.collect()
	};

	let mut allergic_ingredients: HashMap<String, HashSet<String>> = HashMap::new();

	for list in ingredients_lists.iter() {
		for allergen in list.allergens.iter().cloned() {
			match allergic_ingredients.entry(allergen) {
				Entry::Occupied(mut entry) => {
					let possible_ingredients = entry.get_mut();
					let mut remove_ingredients: Vec<String> = Vec::new();
					for ingredient in possible_ingredients.iter() {
						if !list.ingredients.contains(ingredient) {
							remove_ingredients.push(ingredient.clone());
						}
					}
					for ingredient in remove_ingredients.iter() {
						possible_ingredients.remove(ingredient);
					}
				}
				Entry::Vacant(entry) => {
					entry.insert(list.ingredients.iter().map(|s| s.to_owned()).collect());
				}
			}
		}
	}

	let mut allergy_free_ingredients: HashMap<String, usize> = HashMap::new();
	for list in ingredients_lists.iter() {
		for ingredient in list.ingredients.iter().cloned() {
			let ingredient_entry = allergy_free_ingredients.entry(ingredient).or_insert(0);
			*ingredient_entry += 1;
		}
	}

	for possible_allergy_ingredients in allergic_ingredients.values() {
		for ingredient in possible_allergy_ingredients.iter() {
			allergy_free_ingredients.remove(ingredient);
		}
	}

	let mut definite_allergens: HashMap<String, String> = HashMap::new();
	for (allergen, ingredients) in allergic_ingredients.iter() {
		let allergic_ingredients: Vec<&String> = ingredients
			.iter()
			.filter(|ingredient| !allergy_free_ingredients.contains_key(*ingredient))
			.collect();
		if allergic_ingredients.len() == 1 {
			definite_allergens.insert(allergen.clone(), allergic_ingredients[0].to_owned());
		}
	}
	let mut definite_allergens_ingredients: HashSet<String> = definite_allergens.values().cloned().collect();
	loop {
		let mut updated_allergens = false;
		for (allergen, ingredients) in allergic_ingredients.iter() {
			if definite_allergens.contains_key(allergen) {
				continue;
			}
			let mut remaining_ingredients: Vec<&String> = Vec::new();
			for ingredient in ingredients.iter() {
				if !allergy_free_ingredients.contains_key(ingredient) && !definite_allergens_ingredients.contains(ingredient) {
					remaining_ingredients.push(ingredient);
				}
			}
			if remaining_ingredients.len() == 1 {
				definite_allergens.insert(allergen.clone(), remaining_ingredients[0].clone());
				definite_allergens_ingredients.insert(remaining_ingredients[0].clone());
				updated_allergens = true;
			}
		}
		if !updated_allergens {
			break;
		}
	}

	let mut definite_allergens_sorted: Vec<String> = definite_allergens.keys().cloned().collect();
	definite_allergens_sorted.sort_unstable();
	let mut definite_allergens_ingredients: Vec<String> = Vec::with_capacity(definite_allergens_sorted.len());
	for allergen in definite_allergens_sorted.iter() {
		definite_allergens_ingredients.push(definite_allergens.get(allergen).unwrap().clone());
	}
	println!("{}", definite_allergens_ingredients.join(","));
}
