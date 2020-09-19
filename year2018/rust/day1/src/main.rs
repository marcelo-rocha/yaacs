use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
//use std::process::exit;

fn read_file(name: &str) -> Result<i32, &'static str>{
    // File hosts must exist in current path before this produces output
    let mut r:i32 = 0;
    if let Ok(lines) = read_lines(name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                if s.len() <= 1 {
                    return Err("invalid line");
                }
                let num = s.parse::<i32>().unwrap();
                r += num;
            }
        }
        Ok(r)
    } else {
        Err("read error")
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Inform the input file path!");
        std::process::exit(-1);
    }
    if let Ok(r) = read_file(&args[1]) {
        println!("Total is {}", r);
    } else {
        println!("read error");
    }
}
