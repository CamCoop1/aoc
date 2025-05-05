use utils::{file_reader, harness::Solve};

pub struct D01;

 
fn count_characters(s : &str, character : char ) -> i16 {
    s.chars().filter(|&c| c == character).count() as i16
}
 

fn determine_floor(input : &str) -> i16 { 
    let go_up: i16 = count_characters(input, '(');
    let go_down: i16 = count_characters(input, ')');

    go_up - go_down
    
}

impl Solve for D01 {
    fn part1(_: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut counter:  i16 = 0;

        for i in &input {
            let floor = determine_floor(&i);
            counter += floor
        }

        counter.to_string()
        
    }

    fn part2(_: String, path: &String) -> String {
        let input = file_reader::read_lines(path);
        let mut counter:  i16 = 0;

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