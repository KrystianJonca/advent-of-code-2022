use std::fs;

fn main() {
  let input = fs::read_to_string("input.txt").expect("Unable to open input file!");

  let rucksacks = input.split("\n");

  let mut common_items_prioriy_sum = 0;

  for rucksack in rucksacks {
    let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

    let common_compartments_items = get_common_items(first_compartment, second_compartment);

    for item in common_compartments_items.chars() {
      common_items_prioriy_sum += get_item_priority(item).expect("Invalid item!");
    }
  }

  println!("Common items priority sum: {}", common_items_prioriy_sum);
}

fn get_item_priority(item: char) -> Option<u32> {
  let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

  for (index, letter) in alphabet.iter().enumerate() {
    if item == *letter {
      return Some(index as u32 + 1);
    }
  }
  None
}

fn get_common_items(first_compartment: &str, second_compartment: &str) -> String {
  let mut common_items = String::new();

  for item in first_compartment.chars() {
    if second_compartment.contains(item) {
      if !common_items.contains(item) {
        common_items.push(item);
      }
    }
  }

  common_items
}