#![allow(dead_code, unused)]

use std::{collections::HashMap, path::Component};

use utils::{Result, harness::Solve};


trait Operations {
    fn assignment (&mut self, value : u16, name : String);

    fn apply_and (&mut self, x : &str, y : &str, name : String);

    fn apply_or (&mut self, x : &str, y : &str, name : String);
    
}

impl  Operations for HashMap<String, u16> {
    fn apply_and (&mut self, x : &str, y : &str, name :String) {
        self.insert(name, self.get(x).unwrap() & self.get(y).unwrap());    
    }

    fn apply_or (&mut self, x : &str, y : &str, name : String) {
        self.insert(name, self.get(x).unwrap() | self.get(y).unwrap());    
    }

    fn assignment (&mut self, value : u16, name : String) {
        self.insert(name, value);    
    }
}


pub struct D07;

impl Solve for D07 {
    fn part1(input: String, path: &String) -> String {
        let mut connections : HashMap<String, u16>= HashMap::new();
        connections.assignment(123, "x".to_string());
        connections.assignment(456, "y".into());
        connections.apply_and("x", "y", "d".into());
        // connections.insert("x", 123);
        // connections.insert("y", 456);
        // connections.insert("d", connections.get("x").unwrap() & connections.get("y").unwrap());
        // connections.insert("e", connections.get("x").unwrap() | connections.get("y").unwrap());
        // connections.insert("f", connections.get("x").unwrap() << 2 );
        // connections.insert("g", connections.get("y").unwrap() >> 2);
        // connections.insert("h", !connections.get("x").unwrap());
        // connections.insert("i", !connections.get("y").unwrap());


        println!("{:?}", connections);

        String::new()
    }

    fn part2(input: String, path: &String) -> String {
        todo!()
    }
}