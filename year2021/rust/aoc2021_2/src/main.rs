use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let filename: String = args().nth(1).unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut horizontal_pos: i32 = 0;
    let mut depth_pos: i32 = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); 
        let mut iter = line.split_whitespace();
        let dir = iter.next();
        let value = iter.next().unwrap().parse::<i32>().unwrap(); 
        match dir.unwrap() {
            "forward" => horizontal_pos += value,
            "up" => depth_pos -= value,
            "down" => depth_pos += value,
            _ => println!("ops")

        }

    }
    println!("horizontal is {}, depth is {}, total is {}", horizontal_pos, depth_pos, horizontal_pos * depth_pos);

    
}
