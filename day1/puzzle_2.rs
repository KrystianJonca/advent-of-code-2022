use std::fs::File;
use std::io::{ BufReader, BufRead, Error };

fn main() -> Result<(), Error> {
  let path = "input.txt";

  let input = File::open(path).expect("Unable to open input file!");
  let buffered = BufReader::new(input);

  let mut top_3_most_calories_caried: [u32; 3] = [0; 3];
  let mut calories_caried = 0;

  for line in buffered.lines() {
    match line?.parse::<u32>() {
      Ok(n) => {
        calories_caried += n;
      },
      Err(_e) => {
        if calories_caried > top_3_most_calories_caried[0] {
          top_3_most_calories_caried[2] = top_3_most_calories_caried[1];
          top_3_most_calories_caried[1] = top_3_most_calories_caried[0];
          top_3_most_calories_caried[0] = calories_caried;
        } else if calories_caried > top_3_most_calories_caried[1] && calories_caried < top_3_most_calories_caried[0] {
          top_3_most_calories_caried[2] = top_3_most_calories_caried[1];
          top_3_most_calories_caried[1] = calories_caried;
        } else if calories_caried > top_3_most_calories_caried[2] && calories_caried < top_3_most_calories_caried[1] {
          top_3_most_calories_caried[2] = calories_caried;
        }
        
        calories_caried = 0;
      },
    }
  }
  let total_top_3_calories: u32 = top_3_most_calories_caried.iter().sum();

  println!("Top 3 most calories caried sum: \n {}", total_top_3_calories);

  Ok(())
}