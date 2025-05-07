use rayon::prelude::*;
use utils::{file_reader, harness::Solve, Result};
use md5;

pub struct D04;

impl Solve for D04 {
    fn part1(input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut counter: i32 = 0;
        for i in input {
            loop {
                let iteration_input = format!("{}{}", i, counter);
                let input_bytes = iteration_input.as_bytes();
                let digest = md5::compute(input_bytes);
                let hex_string = format!("{:x}", digest);
                if hex_string.starts_with("00000") {
                    break
                }
                counter +=1                 
            }
        
        }
        counter.to_string()
    }

    fn part2(input: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut counter: i64 = 0;
        for i in input {
            loop {
                let iteration_input = format!("{}{}", i, counter);
                let input_bytes = iteration_input.as_bytes();
                let digest = md5::compute(input_bytes);
                let hex_string = format!("{:x}", digest);
                if hex_string.starts_with("000000") {
                    break
                }
                counter +=1                 
            }
        
        }
        counter.to_string()
    }
}

