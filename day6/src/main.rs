use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to open input file!");

    let result = 
    input.as_bytes()
        .windows(4)
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(idx, c)| !window[..idx].contains(c))
        })
        .expect("Marker not found!")
        + 4;

    println!("{}", result);

    let result_part2 = 
    input.as_bytes()
        .windows(14)
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(idx, c)| !window[..idx].contains(c))
        })
        .expect("Marker not found!")
        + 14;

    println!("{}", result_part2);
}
