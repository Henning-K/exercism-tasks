use std::collections::{HashMap, HashSet};
use std::char;

struct DigitCombinations {
    curr: u64,
    limit: u64,
}

impl DigitCombinations {
    fn new(length: u8) -> Self {
        let (curr, limit) = match length {
            1 => (0u64, 9u64),
            2 => (10u64, 98u64),
            3 => (102u64, 987u64),
            4 => (1023u64, 9876u64),
            5 => (10234u64, 98765u64),
            6 => (102345u64, 987654u64),
            7 => (1023456u64, 9876543u64),
            8 => (10234567u64, 98765432u64),
            9 => (102345678u64, 987654321u64),
            10 => (1023456789u64, 9876543210u64),
            _ => panic!("Invalid length for DigitCombinations."),
        };
        DigitCombinations {
            curr: curr,
            limit: limit,
        }
    }
}

fn has_dups(inp: &str) -> bool {
    inp.chars().any(|e| inp.chars().filter(|&c| e == c).count() > 1)
}

impl Iterator for DigitCombinations {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut curr_string = format!("{}", self.curr);
        while has_dups(&curr_string) && self.curr <= self.limit {
            self.curr += 1;
            curr_string = format!("{}", self.curr);
        }
        self.curr += 1;
        if self.curr > self.limit {
            return None;
        }
        Some(curr_string)
    }
}

fn is_solution(puzzle: &str, pos_solution: &HashMap<char, u8>) -> bool {
    // parsing phase:
    let puzzle: String = puzzle.chars()
        .map(|c| if let Some(&d) = pos_solution.get(&c) {
            char::from_digit(d as u32, 10).unwrap()
        } else {
            c
        })
        .collect();
    // any leading zeros?
    if puzzle.replace("+", " ")
        .replace("==", " ")
        .split_whitespace()
        .any(|elem| elem.chars().nth(0).unwrap() == '0') {
        return false;
    }
    let temp: Vec<&str> = puzzle.split(" == ").collect();
    let equation = temp[0];
    let result = temp[1].parse::<u32>().unwrap();
    let addends: Vec<u32> = equation.split(" + ").map(|e| e.parse::<u32>().unwrap()).collect();
    let sum: u32 = addends.iter().sum();
    sum == result
}

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let letters: HashSet<char> = puzzle.chars().filter(|&c| c.is_uppercase()).collect();
    if letters.len() > 10 {
        return None; // Number of distinct letter can not be greater than 10.
    }

    for comb in DigitCombinations::new(letters.len() as u8) {
        let comb: Vec<u8> = comb.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let pos_solution: HashMap<char, u8> =
            letters.iter().zip(comb.iter()).map(|(&c, &d)| (c, d)).collect();
        if is_solution(puzzle, &pos_solution) {
            return Some(pos_solution);
        }
    }

    None
}
