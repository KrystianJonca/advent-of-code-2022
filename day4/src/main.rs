use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to open input file!");

    let mut sections_overlap_count = 0;
    let mut sections_overlap_partially_count = 0;

    for line in input.lines() {
       let (first_section_assignment, second_section_assignment) = get_section_assignments(line).expect("Invalid input!");

        if sections_overlap_entirely(first_section_assignment, second_section_assignment) {
            sections_overlap_count += 1;
        }

        if sections_overlap(first_section_assignment, second_section_assignment) {
            sections_overlap_partially_count += 1;
        }
    }

    println!("Number of sections overlapping, {}", sections_overlap_count);
    println!("Number of sections overlapping partially, {}", sections_overlap_partially_count);

}

fn sections_overlap_entirely(first_section: &str, second_section: &str) -> bool {
    let (first_section_range_start, first_section_range_end) = get_section_range(first_section).expect("Invalid input!");
    let (second_section_range_start, second_section_range_end) = get_section_range(second_section).expect("Invalid input!");

    // checks if the first section range is within the second section range or vice-versa
    if first_section_range_start >= second_section_range_start && first_section_range_end <= second_section_range_end || second_section_range_start >= first_section_range_start && second_section_range_end <= first_section_range_end {
        return true;
    }

    false
}

fn sections_overlap(first_section: &str, second_section: &str) -> bool {
    let (first_section_range_start, first_section_range_end) = get_section_range(first_section).expect("Invalid input!");
    let (second_section_range_start, second_section_range_end) = get_section_range(second_section).expect("Invalid input!");

    let first_section_range = first_section_range_start..=first_section_range_end;
    let second_section_range = second_section_range_start..=second_section_range_end;

    if first_section_range.contains(&second_section_range_start) || first_section_range.contains(&second_section_range_end) || second_section_range.contains(&first_section_range_start) || second_section_range.contains(&first_section_range_end){
        return true;
    }

    false
}

fn get_section_range(line: &str) -> Option<(u32, u32)> {
    let section_range: Vec<&str> = line.split("-").collect();

    if section_range.len() != 2 {
        return None;
    }

    let section_range_start = section_range[0].parse::<u32>().expect("Invalid input!");
    let section_range_end = section_range[1].parse::<u32>().expect("Invalid input!");

    
    Some((section_range_start, section_range_end))
}

fn get_section_assignments(line: &str) -> Option<(&str, &str)> {
    let section_assignments: Vec<&str> = line.split(",").collect();

    if section_assignments.len() != 2 {
        return None;
    }

    let first_section_assignment = section_assignments[0];
    let second_section_assignment = section_assignments[1];

    Some((first_section_assignment, second_section_assignment))
}
