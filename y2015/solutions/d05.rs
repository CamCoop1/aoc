#![allow(dead_code, unused, non_snake_case)]

use utils::{Result, file_reader, harness::Solve};
use itertools::Itertools;

const VOWELS : [char;5] = ['a', 'e', 'i', 'o', 'u'];
const DISALLOWED : [&str; 4] = ["ab", "cd", "pq", "xy"];


fn check_for_double_letters(input : &str) -> bool {
    input
        .chars()
        .tuple_windows::<(_,_)>() // Take a two element window and check if each char is the same
        .any(|(a,b)| a==b)
}

// Need a function that will check if any pairs of two letters appear twice without overlap
// e.g xyxy but not aaa
fn has_repeated_pair(input: &str) -> bool {
    let windowed_pairs = input.chars().tuple_windows::<(_, _)>();

    for (i, (a, b)) in windowed_pairs.enumerate() {
        let window = format!("{}{}", a, b);

        // Collect non-overlapping 2-char chunks (e.g. "xyxy" -> ["xy", "xy"])
        let chunks: Vec<_> = input
            .chars()
            .chunks(2)
            .into_iter()
            .map(|mut ch| {
                let first = ch.next().unwrap_or('\0');
                let second = ch.next().unwrap_or('\0');
                format!("{}{}", first, second)
            })
            .collect();

        // Count how many times the window appears in the non-overlapping chunks
        let count = chunks.iter().filter(|c| *c == &window).count();

        if count > 1 {
            return true;
        }
    }

    false
}

fn check_repeat_letter_with_buffer_element(input : &str) -> bool {
    input
        .chars()
        .tuple_windows::<(_,_,_)>()
        .any(|(a, b, c)|a==c && (a!=b && c!=b))
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

fn process_input_for_nice_string_part2(input : &str) -> bool {
    let check_repeat_with_buffer = check_repeat_letter_with_buffer_element(&input);
    let check_repeated_pair = has_repeated_pair(&input);

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
