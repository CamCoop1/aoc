use std::sync::Arc;

use utils::{file_reader, harness::Solve};

pub struct D01;

/// Count the number of characters in a string slice
fn count_characters(s : &str, character : char ) -> i16 {
    s.chars().filter(|&c| c == character).count() as i16
}
 
/// Determine the floor based on the up and down characters
fn determine_floor(input : &str) -> i16 { 
    let go_up: i16 = count_characters(input, '(');
    let go_down: i16 = count_characters(input, ')');

    go_up - go_down
    
}

impl Solve for D01 {
    fn part1(_: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let counter:  i16 = input
            .into_iter()
            .map(|i| determine_floor(&i))
            .sum();

        counter.to_string()        
    }

    fn part2(_: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut counter : i16 = 0;
        
        for i in &input {
            for (position, character) in i.chars().into_iter().enumerate() {
                let mapped_position = position + 1;
                counter += determine_floor(&character.to_string());
                if counter == -1 {
                    return mapped_position.to_string()
                }
            }
         

        }

        counter.to_string()
        
    }
}