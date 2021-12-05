use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

const BITS_QTY: usize = 12;

fn main() {
    let filename: String = args().nth(1).unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut counters: [i32; BITS_QTY] = [0; BITS_QTY];
    let mut lines_qty: i32 = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); 
        lines_qty += 1;
        let value = isize::from_str_radix(line.as_str(), 2).unwrap();
        let mut bit_mask = 1;
        for i in 0..BITS_QTY {
            if (value & bit_mask) != 0 {
                counters[i] += 1;
            }
            bit_mask = bit_mask << 1;
        }
    }
    let threshold = lines_qty / 2;
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;
    let mut bit_mask = 1;

    for i in 0..BITS_QTY {
        if counters[i] >= threshold {
            gamma += bit_mask;
        } else {
            epsilon += bit_mask;
        }
        bit_mask = bit_mask << 1;
    }

    println!("gamma is {}, epsilon is {}, power consumption is {}", gamma, epsilon, gamma * epsilon);  
}
