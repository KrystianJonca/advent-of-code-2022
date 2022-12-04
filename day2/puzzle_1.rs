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
  fn from_player_move (player_move: &str) -> GameMove {
    match player_move {
      "X" => GameMove::Rock,
      "Y" => GameMove::Paper,
      "Z" => GameMove::Scissors,
      _ => panic!("Invalid player move!"),
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

  let game_moves = input.split("\n");

  let mut player_score = 0;

  for game_move in game_moves {
    let (oponent_move, player_move) = game_move.split_at(1);

    let oponent_move = GameMove::from_oponent_move(oponent_move.trim());
    let player_move = GameMove::from_player_move(player_move.trim());

    let game_result = get_game_result(&oponent_move, &player_move);
    let game_score = player_move.get_move_score() + game_result.get_score();

    player_score += game_score;
  }

  println!("Total player score: {}", player_score);
}

fn get_game_result(first_move: &GameMove, second_move: &GameMove) -> GameResult {
  match (first_move, second_move) {
    (GameMove::Rock, GameMove::Paper) => GameResult::Win,
    (GameMove::Rock, GameMove::Scissors) => GameResult::Loss,
    (GameMove::Paper, GameMove::Rock) => GameResult::Loss,
    (GameMove::Paper, GameMove::Scissors) => GameResult::Win,
    (GameMove::Scissors, GameMove::Rock) => GameResult::Win,
    (GameMove::Scissors, GameMove::Paper) => GameResult::Loss,
    _ => GameResult::Draw,
  }
}