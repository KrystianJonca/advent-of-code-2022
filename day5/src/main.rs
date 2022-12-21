use itertools::Itertools;
use std::fs;

#[derive(Debug)] 
struct Instruction {
    amount: usize,
    from: usize,
    to: usize
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to open input file!");

    let (stacks_input, instructions_input) = input.split_once("\n\n").unwrap();
    let (crates, stacks) = stacks_input.rsplit_once("\n").unwrap();
    let stacks_count = stacks.split_whitespace().count();

    let mut stacks = parse_cargo(crates, stacks_count).expect("Unable to parse cargo");
    let instructions= parse_instructions(instructions_input).expect("Unable to parse instructions");

    for instruction in instructions {
        for _ in 0..instruction.amount {
            move_crate(&mut stacks, &instruction);
        }
    }

    let top_stacks = stacks.iter().fold(String::from(""), |mut acc, stack| {acc.push(*stack.last().unwrap()); acc});
    println!("--- Part 1 ---");
    println!("{:#?}", top_stacks);

    let input = fs::read_to_string("src/input.txt").expect("Unable to open input file!");

    let (stacks_input, instructions_input) = input.split_once("\n\n").unwrap();
    let (crates, stacks) = stacks_input.rsplit_once("\n").unwrap();
    let stacks_count = stacks.split_whitespace().count();

    let mut stacks = parse_cargo(crates, stacks_count).expect("Unable to parse cargo");
    let instructions= parse_instructions(instructions_input).expect("Unable to parse instructions");

    for instruction in instructions {
        move_multiple_creates(&mut stacks, &instruction);
    }

    let top_stacks = stacks.iter().fold(String::from(""), |mut acc, stack| {acc.push(*stack.last().unwrap()); acc});
    println!("--- Part 2 ---");
    println!("{:#?}", top_stacks);
}

fn move_crate(stacks: &mut Vec<Vec<char>>, instruction: &Instruction) {
    let crate_to_move = stacks[instruction.from].pop().unwrap();
    stacks[instruction.to].push(crate_to_move);
}

fn move_multiple_creates(stacks: &mut Vec<Vec<char>>, instruction: &Instruction) {
    let stack_size = stacks[instruction.from].len();
    let crates_to_move = stacks[instruction.from].split_off(stack_size - instruction.amount);

    stacks[instruction.to].extend(crates_to_move);
}

fn parse_cargo(crates: &str, stacks_count: usize) -> Option<Vec<Vec<char>>> {
    let mut stacks = vec![Vec::new(); stacks_count];
    
    for line in crates.lines().rev() {
        for (i, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            let potential_platform = chunk.nth(1)?;
            if potential_platform.is_alphabetic() {
                stacks[i].push(potential_platform);
            }
        }
    }
    Some(stacks)
}

fn parse_instructions(instructions_input: &str) -> Option<Vec<Instruction>> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in instructions_input.lines() {
        let rest = line.strip_prefix("move ")?;
        let (amount, rest) = rest.split_once(" from ")?;
        let (from, to) = rest.split_once(" to ")?;
        let instruction = Instruction {
            amount: amount.parse().ok()?,
            from: from.parse::<usize>().ok()? - 1,
            to: to.parse::<usize>().ok()? - 1,
        };
        instructions.push(instruction);
    }
    Some(instructions)
}