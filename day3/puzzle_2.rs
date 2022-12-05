use std::fs;

fn main() {
  let input = fs::read_to_string("input.txt").expect("Unable to open input file!");

  let rucksacks: Vec<&str> = input.split("\n").collect();

  let mut index = 2;
  let mut group_badge_items_sum = 0;

  while index < rucksacks.len() {
    let first_rucksack = rucksacks[index - 2];
    let second_rucksack = rucksacks[index - 1];
    let third_rucksack = rucksacks[index];

    let group_badge_item = get_group_badge(first_rucksack, second_rucksack, third_rucksack).expect("No group badge item found!");

    group_badge_items_sum += get_item_priority(group_badge_item).expect("Invalid item!");
    index += 3;
  }

  println!("Group badge items priority sum: {}", group_badge_items_sum);
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

fn get_group_badge(first_rucksack: &str, second_rucksack: &str, third_rucksack: &str) -> Option<char> {
  for item in first_rucksack.chars() {
    if second_rucksack.contains(item) && third_rucksack.contains(item) {
      return Some(item);
    }
  }
  None
}
