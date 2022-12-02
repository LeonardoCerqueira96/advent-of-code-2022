use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

use phf::phf_map;

enum RPSShape {
    Rock,
    Paper,
    Scissors,
}

enum RPSAction {
    Lose,
    Draw,
    Win,
}

static OPPONENT_GAME_MAPPING: phf::Map<&'static str, RPSShape> = phf_map!(
    "A" => RPSShape::Rock,
    "B" => RPSShape::Paper,
    "C" => RPSShape::Scissors,
);

static PART1_MY_GAME_MAPPING: phf::Map<&'static str, RPSShape> = phf_map!(
    "X" => RPSShape::Rock,
    "Y" => RPSShape::Paper,
    "Z" => RPSShape::Scissors,
);

static PART2_MY_ACTIONS_MAPPING: phf::Map<&'static str, RPSAction> = phf_map!(
    "X" => RPSAction::Lose,
    "Y" => RPSAction::Draw,
    "Z" => RPSAction::Win,
);

fn parse_input<T: AsRef<Path>>(filename: T) -> io::Result<(Vec<String>, Vec<String>)> {
    let mut opponent_games = Vec::new();
    let mut my_games = Vec::new();

    // Open input file
    let input = File::open(filename)?;
    let input_buf = BufReader::new(input);

    // Read line by line
    for line in input_buf.lines() {
        let line = line?;

        let mut fields_it = line.split_ascii_whitespace().take(2);
        let opponent_game = fields_it.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Expected first column not found",
            )
        })?;
        let my_game = fields_it.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Expected second column not found",
            )
        })?;

        opponent_games.push(opponent_game.to_string());
        my_games.push(my_game.to_string());
    }

    Ok((opponent_games, my_games))
}

fn part1_calculate_scores(opponent_games: &[String], my_games: &[String]) -> u64 {
    assert_eq!(opponent_games.len(), my_games.len());

    let mut total_score = 0;
    for i in 0..my_games.len() {
        let opponent_game = OPPONENT_GAME_MAPPING
            .get(&opponent_games[i] as &str)
            .expect("Unkwown mapping");
        let my_game = PART1_MY_GAME_MAPPING
            .get(&my_games[i] as &str)
            .expect("Unkwown mapping");

        let score = match opponent_game {
            RPSShape::Rock => match my_game {
                RPSShape::Rock => 1 + 3,
                RPSShape::Paper => 2 + 6,
                RPSShape::Scissors => 3,
            },
            RPSShape::Paper => match my_game {
                RPSShape::Rock => 1,
                RPSShape::Paper => 2 + 3,
                RPSShape::Scissors => 3 + 6,
            },
            RPSShape::Scissors => match my_game {
                RPSShape::Rock => 1 + 6,
                RPSShape::Paper => 2,
                RPSShape::Scissors => 3 + 3,
            },
        };

        total_score += score;
    }

    total_score
}

fn part2_calculate_scores(opponent_games: &[String], my_actions: &[String]) -> u64 {
    assert_eq!(opponent_games.len(), my_actions.len());

    let mut total_score = 0;
    for i in 0..my_actions.len() {
        let opponent_game = OPPONENT_GAME_MAPPING
            .get(&opponent_games[i] as &str)
            .expect("Unkwown mapping");
        let my_action = PART2_MY_ACTIONS_MAPPING
            .get(&my_actions[i] as &str)
            .expect("Unkwown mapping");

        let score = match opponent_game {
            RPSShape::Rock => match my_action {
                RPSAction::Lose => 3,
                RPSAction::Draw => 1 + 3,
                RPSAction::Win => 2 + 6,
            },
            RPSShape::Paper => match my_action {
                RPSAction::Lose => 1,
                RPSAction::Draw => 2 + 3,
                RPSAction::Win => 3 + 6,
            },
            RPSShape::Scissors => match my_action {
                RPSAction::Lose => 2,
                RPSAction::Draw => 3 + 3,
                RPSAction::Win => 1 + 6,
            },
        };

        total_score += score;
    }

    total_score
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the input and time it
    let t0 = Instant::now();
    let (opponent_games, my_games) = parse_input("inputs/day02.in")?;
    let parse_time = t0.elapsed();

    // Compute part 1 and time it
    let t1 = Instant::now();
    let part1_total_score = part1_calculate_scores(&opponent_games, &my_games);
    let part1_time = t1.elapsed();

    // Compute part 2 and time it
    let t2 = Instant::now();
    let part2_total_score = part2_calculate_scores(&opponent_games, &my_games);
    let part2_time = t2.elapsed();

    // Print results
    let parse_time =
        parse_time.as_millis() as f64 + (parse_time.subsec_nanos() as f64 * 1e-6).fract();
    println!("Parsing the input took {:.6}ms\n", parse_time);

    let part1_time =
        part1_time.as_millis() as f64 + (part1_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 1:\nTook {:.6}ms\nPart 1 total score: {}\n",
        part1_time, part1_total_score
    );

    let part2_time =
        part2_time.as_millis() as f64 + (part2_time.subsec_nanos() as f64 * 1e-6).fract();
    println!(
        "Part 2:\nTook {:.6}ms\nPart 2 total score: {}\n",
        part2_time, part2_total_score
    );

    Ok(())
}
