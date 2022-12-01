use std::fs::File;
use std::io::{ BufReader, BufRead, Error };

fn main() -> Result<(), Error> {
  let path = "input.txt";

  let input = File::open(path).expect("Unable to open input file!");
  let buffered = BufReader::new(input);

  let mut most_calories_caried = 0;
  let mut calories_caried = 0;

  for line in buffered.lines() {
    match line?.parse::<u32>() {
      Ok(n) => {
        calories_caried += n;
      },
      Err(_e) => {
        if calories_caried > most_calories_caried {
          most_calories_caried = calories_caried;
        }
        calories_caried = 0;
      },
    }
  }

  println!("Most calories caried by an Elf: {most_calories_caried}");

  Ok(())
}