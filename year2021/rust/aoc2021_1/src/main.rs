use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let filename: String = args().nth(1).unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut previous: i32 = -1;
    let mut count: i32 = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); 
        let value = line.parse::<i32>().unwrap();
        if index > 0 {
            if value > previous {
                count += 1;
            } 
            previous = value
        }
    }
    println!("total is {}", count);

    
}
