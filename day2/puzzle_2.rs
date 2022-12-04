use std::fs;

enum GameResult {
  Win,
  Draw,
  Loss,
}

impl GameResult {
  fn get_score(&self) -> u32 {
    match self {
      GameResult::Win => 6,
      GameResult::Draw => 3,
      GameResult::Loss => 0,
    }
  }
  fn from_input (input: &str) -> GameResult {
    match input {
      "X" => GameResult::Loss,
      "Y" => GameResult::Draw,
      "Z" => GameResult::Win,
      _ => panic!("Invalid input!"),
    }
  }
}

enum GameMove {
  Rock,
  Paper,
  Scissors,
}

impl GameMove {
  fn get_move_score (&self) -> u32 {
    match self {
      GameMove::Scissors => 3,
      GameMove::Paper => 2,
      GameMove::Rock => 1,
    }
  }
  fn from_oponent_move (oponent_move: &str) -> GameMove {
    match oponent_move {
      "A" => GameMove::Rock,
      "B" => GameMove::Paper,
      "C" => GameMove::Scissors,
      _ => panic!("Invalid oponent move!"),
    }
  }
}

fn main() {
  let input = fs::read_to_string("input.txt").expect("Unable to open input file!");

  let input_entries = input.split("\n");

  let mut player_score = 0;

  for entry in input_entries {
    let (oponent_move, game_result) = entry.split_at(1);

    let oponent_move = GameMove::from_oponent_move(oponent_move.trim());
    let game_result = GameResult::from_input(game_result.trim());

    let player_move = get_player_move(&oponent_move, &game_result);

    let game_score = player_move.get_move_score() + game_result.get_score();

    player_score += game_score;
  }

  println!("Total player score: {}", player_score);
}

fn get_player_move(oponent_move: &GameMove, game_result: &GameResult) -> GameMove {
  match (oponent_move, game_result) {
    (GameMove::Rock, GameResult::Win) => GameMove::Paper,
    (GameMove::Rock, GameResult::Draw) => GameMove::Rock,
    (GameMove::Rock, GameResult::Loss) => GameMove::Scissors,
    (GameMove::Paper, GameResult::Win) => GameMove::Scissors,
    (GameMove::Paper, GameResult::Draw) => GameMove::Paper,
    (GameMove::Paper, GameResult::Loss) => GameMove::Rock,
    (GameMove::Scissors, GameResult::Win) => GameMove::Rock,
    (GameMove::Scissors, GameResult::Draw) => GameMove::Scissors,
    (GameMove::Scissors, GameResult::Loss) => GameMove::Paper,
  }
}
