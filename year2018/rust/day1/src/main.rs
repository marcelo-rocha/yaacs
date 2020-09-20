use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::env;
use std::collections::HashSet;

fn read_file(name: &str) -> Result<i32, &'static str>{
    // File hosts must exist in current path before this produces output
    let mut freqs = HashSet::<i32>::new();
    let mut f:i32 = 0;
    let file_result = File::open(name);
    if file_result.is_err() {
        return Err("file open error")
    }
    let mut file = file_result.unwrap();
    loop {
        let lines = io::BufReader::new(&file).lines();
        //println!("looping");
        for line in lines {
            if let Ok(s) = line {
                if s.len() <= 1 {
                    return Err("invalid line");
                }
                let num = s.parse::<i32>().unwrap();
                f += num;
                if !freqs.contains(&f) {
                    freqs.insert(f);
                    println!("{} change freq to {}", num, f);
                }
                else {
                    return Ok(f)
                }
            }
        }
        let r = file.seek(SeekFrom::Start(0));
        if r.is_err() {
            return Err("read error");
        }
    } 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Inform the input file path!");
        std::process::exit(-1);
    }
    if let Ok(r) = read_file(&args[1]) {
        println!("First repeated frequency is {}", r);
    } else {
        println!("read error");
    }
}
