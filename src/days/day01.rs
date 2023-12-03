use crate::{Solution, SolutionPair};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::vec::Vec;

///////////////////////////////////////////////////////////////////////////////

lazy_static! {
    static ref REGEXES: Vec<Regex> = vec![
        Regex::new(r"one").unwrap(),
        Regex::new(r"two").unwrap(),
        Regex::new(r"three").unwrap(),
        Regex::new(r"four").unwrap(),
        Regex::new(r"five").unwrap(),
        Regex::new(r"six").unwrap(),
        Regex::new(r"seven").unwrap(),
        Regex::new(r"eight").unwrap(),
        Regex::new(r"nine").unwrap(),
    ];
}

fn find_nums(input: &str) -> (u64, u64) {
    let mut left_digit: Option<(u64, usize)> = None;
    println!("{}", input);

    // scan left to right
    for (size, character) in input.chars().enumerate() {
        if let Some(num) = character.to_digit(10) {
            left_digit = Some((num.try_into().unwrap(), size));
            break;
        }
    }

    let mut right_digit: Option<(u64, usize)> = None;

    // scan right to left
    for (size, character) in input.chars().rev().enumerate() {
        if let Some(num) = character.to_digit(10) {
            right_digit = Some((num.try_into().unwrap(), input.len() - size));
            break;
        }
    }

    // map of found digits
    let mut words: Vec<(usize, u64)> = Vec::new();
    for (size, re) in REGEXES.iter().enumerate() {
        for matches in re.find_iter(input) {
            words.push((matches.end(), (size + 1).try_into().unwrap()));
        }
    }

    words.sort_by(|a, b| a.0.cmp(&b.0));

    // get left digit
    let left_digit_return: u64 = if let Some((value, index)) = left_digit {
        if let Some((first_index, first_value)) = words.first() {
            if index < *first_index {
                value
            } else {
                *first_value
            }
        } else {
            value
        }
    } else {
        words.first().unwrap().1
    };

    // get right digit
    let right_digit_return: u64 = if let Some((value, index)) = right_digit {
        if let Some((last_index, last_value)) = words.last() {
            if index > *last_index {
                value
            } else {
                *last_value
            }
        } else {
            value
        }
    } else {
        words.last().unwrap().1
    };

    println!(
        "{:?} left_digit: {:?} right_digit: {:?}",
        words, left_digit, right_digit
    );
    return (left_digit_return, right_digit_return);
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    let read_result = read_to_string("input/day01.txt");
    let input = match read_result {
        Ok(input) => input,
        Err(_) => {
            println!("Failed to read input file");
            return (Solution::from(sol1), Solution::from(sol2));
        }
    };

    let strings = input.lines().collect::<Vec<_>>();
    let mut sum: u32 = 0;

    for string in strings.iter() {
        // scan left to right
        let mut line_num: u32 = 0;
        for character in string.chars() {
            if let Some(num) = character.to_digit(10) {
                line_num = num * 10;
                break;
            }
        }

        // scan right to left
        for character in string.chars().rev() {
            if let Some(num) = character.to_digit(10) {
                line_num += num;
                break;
            }
        }
        sum += line_num;
    }

    // 2

    let mut next_sum: u64 = 0;

    for string in strings.iter() {
        let (left, right) = find_nums(string);
        println!("left: {}, right: {}", left, right);
        next_sum += (left * 10) + right;
    }

    (Solution::from(sum), Solution::from(next_sum))
}
