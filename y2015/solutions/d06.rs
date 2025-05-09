use std::collections::HashMap;
use itertools::Itertools;
use utils::{file_reader, harness::Solve, Result};
use super::d03::Point;

enum RequestType {
    Toggle,
    TurnOn,
    TurnOff
}

impl TryFrom<&str> for RequestType {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self> {
        let string_components : Vec<&str> = value.split_whitespace().collect();
        if string_components.contains(&"toggle") {
            return Ok(RequestType::Toggle)
        }
        else if string_components.contains(&"off") {
            return Ok(RequestType::TurnOff)
            
        }
        else if string_components.contains(&"on") {
            return Ok(RequestType::TurnOn)
        }
        Err("Could not determine request type for &str into RequestType".into())
        
    }
}

impl RequestType{
    /// Indexes tell us where the grid information is from the string slices
    const TOGGLE_INDEXES : [usize;2] = [1,3]; 
    const TURN_INDEXES : [usize;2] = [2,4];
}
/// Stage1 parser for each command
fn parse_input_string_to_light(input:&str, global_hashset : &mut HashMap<Point, bool>) {
    let request_type = RequestType::try_from(input).expect("");

    match request_type {
        RequestType::Toggle => {
            let (start_light, end_light) = calculate_start_and_end_lights(input, RequestType::TOGGLE_INDEXES).expect("");
            for x in start_light.x..=end_light.x {
                for y in start_light.y..=end_light.y {
                    let current_light = Point::new(x, y);
                    let Some(light) = global_hashset.get(&current_light) else {
                        global_hashset.insert(current_light, true);
                        continue
                    };
                    let toggled_light = !*light;
                    global_hashset.insert(current_light, toggled_light);

                }
            }
        }
        RequestType::TurnOff => {
            let (start_light, end_light) = calculate_start_and_end_lights(input, RequestType::TURN_INDEXES).expect("");
            for x in start_light.x..=end_light.x {
                for y in start_light.y..=end_light.y {
                    let current_light = Point::new(x, y);
                    let Some(_) = global_hashset.get(&current_light) else {
                        global_hashset.insert(current_light, false);
                        continue
                    };
                    global_hashset.insert(current_light, false);

                }
            }
        }
        RequestType::TurnOn => {
            let (start_light, end_light) = calculate_start_and_end_lights(input, RequestType::TURN_INDEXES).expect("");
            for x in start_light.x..=end_light.x {
                for y in start_light.y..=end_light.y {
                    let current_light = Point::new(x, y);
                    let Some(_) = global_hashset.get(&current_light) else {
                        global_hashset.insert(current_light, true);
                        continue
                    };

                    global_hashset.insert(current_light, true);

                }
            }
        }
    }
}
/// Stage 2 parser for each command
fn parse_input_string_to_light_for_brightness(input:&str, global_hashset : &mut HashMap<Point, u64>) {
    let request_type = RequestType::try_from(input).expect("");

    match request_type {
        RequestType::Toggle => {
            let (start_light, end_light) = calculate_start_and_end_lights(input, RequestType::TOGGLE_INDEXES).expect("");
            for x in start_light.x..=end_light.x {
                for y in start_light.y..=end_light.y {
                    let current_light = Point::new(x, y);
                    let Some(light) = global_hashset.get(&current_light) else {
                        global_hashset.insert(current_light, 2);
                        continue
                    };
                    let toggled_light = light + 2;
                    global_hashset.insert(current_light, toggled_light);

                }
            }
        }
        RequestType::TurnOff => {
            let (start_light, end_light) = calculate_start_and_end_lights(input, RequestType::TURN_INDEXES).expect("");
            for x in start_light.x..=end_light.x {
                for y in start_light.y..=end_light.y {
                    let current_light = Point::new(x, y);
                    let Some(brightness) = global_hashset.get(&current_light) else {
                        global_hashset.insert(current_light, 0);
                        continue
                    };
                    let updated_brightness = {match brightness {
                        0 => 0,
                        _ => brightness-1
                    }
                    };
                    global_hashset.insert(current_light, updated_brightness);

                }
            }
        }
        RequestType::TurnOn => {
            let (start_light, end_light) = calculate_start_and_end_lights(input, RequestType::TURN_INDEXES).expect("");
            for x in start_light.x..=end_light.x {
                for y in start_light.y..=end_light.y {
                    let current_light = Point::new(x, y);
                    let Some(brightness) = global_hashset.get(&current_light) else {
                        global_hashset.insert(current_light, 1);
                        continue
                    };

                    global_hashset.insert(current_light, *brightness+1);

                }
            }
        }
    }
}

fn calculate_start_and_end_lights(input : &str, indecies : [usize;2]) -> Result<(Point, Point)>{
    let input_list: Vec<&str> = input.split_whitespace().collect();
    let light1 : Point = input_list[indecies[0]]
                .split(",")
                .chunks(2)
                .into_iter()
                .map(|mut values | Point::new(
                        values.next().expect("").parse::<i64>().expect("Could not parse to i64"),
                        values.next().expect("").parse::<i64>().expect("Could not parse to i64")
                ))
                .collect::<Vec<Point>>()[0].clone();
    
    let light2 : Point = input_list[indecies[1]]
        .split(",")
        .chunks(2)
        .into_iter()
        .map(|mut values | Point::new(
                values.next().expect("").parse::<i64>().expect(""),
                values.next().expect("").parse::<i64>().expect("")
        ))
        .collect::<Vec<Point>>()[0].clone();

    return Ok((light1 ,light2 ))

}


pub struct D06;



impl Solve for D06  {
    fn part1(_: String, path: &String) -> String {
        let mut global_hashset : HashMap<Point, bool> = HashMap::new();
        let input = file_reader::read_lines(path);
        for i in input {
            parse_input_string_to_light(&i, &mut global_hashset);
        }
        global_hashset.into_iter()
            .filter(|(_, is_on)| *is_on)
            .collect::<HashMap<Point, bool>>()
            .len()
            .to_string()
    }

    fn part2(_: String, path: &String) -> String {
        let mut global_hashset : HashMap<Point, u64> = HashMap::new();
        let input = file_reader::read_lines(path);
        for i in input {
            parse_input_string_to_light_for_brightness(&i, &mut global_hashset);
        }
        global_hashset.values()
            .into_iter()
            .sum::<u64>()
            .to_string()
    }
}