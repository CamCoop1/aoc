#![allow(dead_code, unused, non_snake_case)]
use rayon::prelude::*;
use utils::{Result, file_reader, harness::Solve};
use itertools::Itertools;

const VOWELS : [char;5] = ['a', 'e', 'i', 'o', 'u'];
const DISALLOWED : [&str; 4] = ["ab", "cd", "pq", "xy"];

// Part 1 Functional Solutions
fn check_for_double_letters(input : &str) -> bool {
    input
        .chars()
        .tuple_windows::<(_,_)>() // Take a two element window and check if each char is the same
        .any(|(a,b)| a==b)
}

fn process_input_for_nice_string_part1(input : &str) -> bool {
    let check_disallowed : bool = DISALLOWED
        .into_iter()
        .any(|c| input.contains(c));

    // If disallowed is found exit early
    if check_disallowed {
        return false
    }
    
    let check_double_letters : bool = check_for_double_letters(input);

    let check_vowels: bool = {input.chars()
        .into_iter()
        .filter(|c| VOWELS.contains(&c))
        .collect::<Vec<_>>()
        .len() > 2 }; // this evaluation is returns the bool result


    return check_vowels && check_double_letters;
}

// Part 2 Functional Solutions
/// Checking if a repeated pair exists
/// 
/// qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
/// 
fn check_for_repeated_pair(input: &str) -> bool {
    // Create a vector if windowed pairs
    let windowed_pairs: Vec<(char,char)> = input.chars().tuple_windows::<(_, _)>().collect();

    // Iterate through each pair
    for (i, pair) in windowed_pairs.iter().enumerate() {
        // Create a cloned vector of pairs that we will use for comparison
        let mut other_pairs = windowed_pairs.clone();
        // Remove in reverse order to not shift indices
        // Removing the next windowed pair as to not include overlaps
        if i + 1 < other_pairs.len() {
                other_pairs.remove(i + 1);
        }
        other_pairs.remove(i); // always safe, i < len
        // If not the first pair, remove the n-1 pair as to not include overlaps
        if i > 0 {
            other_pairs.remove(i - 1);
        }

        if other_pairs.contains(pair) {
            return true;
        }
    }
    false
}
/// Check for a repeated letter with one element buffer 
/// 
/// xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
fn check_repeat_letter_with_buffer_element(input : &str) -> bool {

    input
        .chars()
        .tuple_windows::<(_,_,_)>()
        .any(|(a, b, c)|a==c && (a!=b && c!=b))
}

fn process_input_for_nice_string_part2(input : &str) -> bool {
    let check_repeat_with_buffer = check_repeat_letter_with_buffer_element(&input);
    let check_repeated_pair = check_for_repeated_pair(&input);
    return check_repeat_with_buffer && check_repeated_pair
}

pub struct D05;

impl Solve for D05 {
    fn part1(_: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let resultant : Vec<String> = input
            .into_iter()
            .filter(|i| process_input_for_nice_string_part1(i))
            .collect();

        resultant.len().to_string()
    }

    fn part2(input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let resultant : Vec<String> = input
            .into_iter()
            .filter(|i| process_input_for_nice_string_part2(i))
            .collect();

        resultant.len().to_string()
    }
}
