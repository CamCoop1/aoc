use std::collections::HashSet;
use std::hash::Hash;
use utils::{file_reader, harness::Solve, Result};

#[derive(Clone, Eq, Copy, PartialEq, Hash)]
pub struct Point {
    pub x : i64,
    pub y : i64
}

impl Default for Point {
    fn default() -> Self {
        Self::origin()
    }
}

impl Point {
    
    fn origin() -> Self {
        Self {
            x : 0,
            y : 0
        }
    }
    
    pub fn new(x_val: i64,y_val: i64) -> Self {
        Self{
            x: x_val,
            y:y_val
        }
    }

    fn update_point(&mut self, updater : &Point) {
        self.x += updater.x;
        self.y += updater.y;
    }
}

#[derive(Clone)]
struct Grid(HashSet<Point>);

impl Default for Grid {
    fn default() -> Self {
        Self(HashSet::from([Point::origin()]))
    }
}

impl Grid{

    fn update_grid(&mut self, updater : &Point) {
        self.0.insert(updater.clone());
    }
}

#[derive(Default)]
struct Controller {
    grid : Grid,
    cursor : Point
}

impl Controller {

    fn process_point(&mut self, updater : Point) {
        self.update_cursor(&updater);
        
        let cursor = self.cursor.clone();
        
        self.update_grid_with_point(&cursor);
    }

    fn update_cursor(&mut self, updater : &Point) {
        self.cursor.update_point(updater);
    }

    fn update_grid_with_point(&mut self, updater : &Point) {
        self.grid.update_grid(updater);
    }

    fn calculate_lucky_houses(&self) -> u32 {
        self.grid.0.len() as u32
    }
}


/// Try mapping a string slice to a Point
fn try_map_str_to_point(input : &str ) -> Result<Point> {
    
    match input {
        "^" => Ok(Point::new(0, 1)),
        ">" => Ok(Point::new(1,0)),
        "v" => Ok(Point::new(0, -1)),
        "<" => Ok(Point::new(-1, 0)),
        _ => Err("Unknown cardinal string slice".into())
    }
}

pub struct D03;

impl Solve for D03 {
    fn part1(_: String, path : &String) -> String {
        let mut controller = Controller::default();
        let input = file_reader::read_lines(path);

         // Loop through inputs line by line 
        input.into_iter()
            .for_each(|i| { 
                i.chars()// for each String line we want to break it into characters
                .into_iter()
                .for_each(|c| { // For each character we map it to a Point and updater our controller
                    let updater_point = try_map_str_to_point(&c.to_string()).expect("Could not map char to Point");
                    controller.process_point(updater_point);
                });                
            });

        controller.calculate_lucky_houses().to_string()

    }

    fn part2(_:String, path : &String) -> String {
        let mut santa_controller = Controller::default();
        let mut robo_santa_controller = Controller::default();
        let input = file_reader::read_lines(path);

        for i in input {
            for (index, c) in i.chars().into_iter().enumerate() {
                if index%2 == 0 {
                    // robo santa as these are even
                    // parse each char of the string into Points
                    let updater_point = try_map_str_to_point(&c.to_string()).expect("Could not map char to Point");
                    // Move by offset, track current position with a some counter 
                    robo_santa_controller.process_point(updater_point);
                }
                else {
                    // normal santa as these are odd
                    // parse each char of the string into Points
                    let updater_point = try_map_str_to_point(&c.to_string()).expect("Could not map char to Point");
                    // Move by offset, track current position with a some counter 
                    santa_controller.process_point(updater_point);                    
                }
            }
        }
        // we minus one lazily here as robo santa and normal santa start at the same house and so 
        // this house needs to not be double counted
        let mut all_houses: HashSet<&Point> = HashSet::new();
        all_houses.extend(santa_controller.grid.0.iter());
        all_houses.extend(robo_santa_controller.grid.0.iter());
      
        all_houses.len().to_string()


    }
}

// #[cfg(test)]
// mod test{
//     use super::*;

//     fn test_part2() {
//         D03::part2(input, path)
//     }
// }