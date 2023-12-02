use crate::{Solution, SolutionPair};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::vec::Vec;

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Color {
    Blue,
    Red,
    Green,
}

type Reveal = (Color, u32);

#[derive(Debug)]
struct Game {
    rounds: Vec<Vec<Reveal>>,
}

lazy_static! {
    static ref COLORS: HashMap<String, Color> = HashMap::from([
        (String::from("blue"), Color::Blue),
        (String::from("red"), Color::Red),
        (String::from("green"), Color::Green)
    ]);
}

fn parse_games(input: &String) -> Vec<Game> {
    let strings: Vec<&str> = input.lines().collect::<Vec<_>>();
    let mut output = Vec::new();
    for line in strings.iter() {
        let re = Regex::new(r"Game \d+: ").unwrap();
        let mut game_with_prefix = re.split(line);
        game_with_prefix.next();
        let game_without_prefix = match game_with_prefix.next() {
            Some(val) => val,
            None => continue,
        };
        let rounds = game_without_prefix.split("; ");

        let mut game: Game = Game { rounds: Vec::new() };
        for round in rounds {
            let mut round_vec: Vec<Reveal> = Vec::new();
            let color_counts = round.split(", ").collect::<Vec<&str>>();
            for color_count in color_counts {
                let mut color_count_split = color_count.split(" ");
                let count = color_count_split.next().unwrap().parse::<u32>().unwrap();
                let color_str = color_count_split.next().unwrap();
                let color = COLORS.get(color_str).unwrap().clone();
                round_vec.push((color, count));
            }
            game.rounds.push(round_vec);
        }
        output.push(game);
    }
    return output;
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    let read_result = read_to_string("input/day02.txt");
    let input = match read_result {
        Ok(input) => input,
        Err(_) => {
            println!("Failed to read input file");
            return (Solution::from(sol1), Solution::from(sol2));
        }
    };
    parse_games(&input);

    let mut sum_one: u32 = 0;
    for (id, game) in parse_games(&input).iter().enumerate() {
        let mut is_game_valid = true;
        for round in game.rounds.clone().into_iter() {
            let mut round_counts: HashMap<Color, u32> = HashMap::new();
            for (color, count) in round {
                *round_counts.entry(color).or_insert(0) += count;
            }

            if *round_counts.entry(Color::Red).or_insert(0) > 12
                || *round_counts.entry(Color::Green).or_insert(0) > 13
                || *round_counts.entry(Color::Blue).or_insert(0) > 14
            {
                is_game_valid = false;
                break;
            }
        }

        if is_game_valid {
            let actual_id: u32 = (id + 1) as u32;
            sum_one += actual_id;
        }
    }

    let mut sum_two: u32 = 0;
    for game in parse_games(&input).into_iter() {
        let mut game_minimums: HashMap<Color, u32> = HashMap::new();
        for round in game.rounds.clone().into_iter() {
            for (color, count) in round {
                game_minimums
                    .entry(color)
                    .and_modify(|current| *current = max(*current, count))
                    .or_insert(count);
            }
        }
        println!("{:?}", game_minimums);

        sum_two += *game_minimums.entry(Color::Red).or_insert(0)
            * *game_minimums.entry(Color::Green).or_insert(0)
            * *game_minimums.entry(Color::Blue).or_insert(0);
    }

    (Solution::from(sum_one), Solution::from(sum_two))
}
