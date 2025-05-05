#![allow(dead_code, non_snake_case, unused)]

use rayon::prelude::*;
use utils::{file_reader, harness::Solve};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct D02;



struct Rectangle {
    length : u64,
    height : u64, 
    width : u64
}

impl Rectangle{
    fn try_from(inputs : [&str; 3]) -> Result<Self> {
        // Try from because what if we cannot parse the 
        // slice into a u16?
        Ok(Self {
            length : inputs[0].parse()?,
            height : inputs[1].parse()?,
            width : inputs[2].parse()?
        })
    }
}


impl Rectangle{
    /// Determing the total suface area of rectangular box
    fn calculate_surface_area(&self) -> u64 {
        2*self.length*self.width + 2*self.width*self.height + 2*self.length*self.height
    }
    /// Determine the smallest two measurements
    fn determine_smallest_combination_of_measurements(&self) -> (u64, u64) {
        let mut measurements = [self.length, self.width, self.height];
        measurements.sort();
        (measurements[0], measurements[1])
    }
    /// Determine the smallest side and its individual area
    /// using the two smallest measurements
    fn calculate_smallest_surface(&self) -> u64 {
        let (m1, m2) = self.determine_smallest_combination_of_measurements();
        m1*m2
    }
    /// Determine the required wrapper paper
    fn required_wrapping_paper(&self) -> u64 {
        let mut surface_area = self.calculate_surface_area();
        surface_area += self.calculate_smallest_surface();
        surface_area
    }
}


impl Rectangle {
    fn volume(&self) -> u64 {
        self.length * self.width * self.height
    }

    fn calculate_ribbon_for_perimeter(&self) -> u64 {
        let (m1, m2) = self.determine_smallest_combination_of_measurements();
        2*m1 + 2*m2
    }

    fn required_ribbon(&self) -> u64 {
        let mut ribbon_length = self.calculate_ribbon_for_perimeter();
        ribbon_length += self.volume();
        ribbon_length
    }
}


impl Solve for D02 {
    fn part1(_: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut required_paper : u64 = 0;

        for i in &input {
            let measurements : Vec<&str> = i.split("x").collect();
            let rectangle = Rectangle::try_from(
                [measurements[0], measurements[1],measurements[2]]
            ).expect("Could not parse measurements into Rectangle struct");

            required_paper += rectangle.required_wrapping_paper();
        }

        required_paper.to_string()
    }

    fn part2(_: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut required_ribbon : u64 = 0;

        required_ribbon += input.into_iter()
            .map(|i| {
                let measurements : Vec<&str> = i.split("x").collect();
                let rectangle = Rectangle::try_from(
                [measurements[0], measurements[1],measurements[2]]
                ).expect("Could not parse measurements into Rectangle struct");

                rectangle.required_ribbon()
            })
            .sum::<u64>();

        required_ribbon.to_string()
    }
}