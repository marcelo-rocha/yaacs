use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

const BITS_QTY: i32 = 12;

fn process_list(src: &Vec<isize>, bit_number: i32, most_common: bool) -> Vec<isize> {
    let mut one_list: Vec<isize> = Vec::new();
    let mut zero_list: Vec<isize> = Vec::new();

    let bit_shift = (BITS_QTY - bit_number) as isize;
    let bit_mask: isize = 1 << bit_shift;

    for value in src {
        if (value & bit_mask) != 0 {
            one_list.push(*value)
        } else {
            zero_list.push(*value)
        }
    }

    if most_common {
        if one_list.len() > zero_list.len() {
            one_list
        } else if zero_list.len() > one_list.len() {
            zero_list
        } else {
            one_list
        }
    } else {
        if one_list.len() < zero_list.len() {
            one_list
        } else if zero_list.len() < one_list.len() {
            zero_list
        } else {
            zero_list
        }
    }
}

enum RatingKind {
    OxygenGeneratorRating,
    CO2ScrubberRating
}

fn extract_rating(values: &Vec<isize>, kind: RatingKind) -> Option<isize> {
    let most_common = if matches!(kind, RatingKind::OxygenGeneratorRating) { true } else { false };
    let mut src = values;
    let mut r;
    for i in 1..BITS_QTY+1 {
        r = process_list(src, i, most_common);
        if r.len() == 1 {
            return Some(r[0])
        }
        src = &r
    }
    return None
}


fn main() {
    let filename: String = args().nth(1).unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut src_list: Vec<isize> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); 
        let value = isize::from_str_radix(line.as_str(), 2).unwrap();
        src_list.push(value);
    }

    let oxygen_rating = extract_rating(&src_list, RatingKind::OxygenGeneratorRating);
    let co2_rating = extract_rating(&src_list, RatingKind::CO2ScrubberRating);

    let v1 = oxygen_rating.unwrap();
    let v2 = co2_rating.unwrap();
    let r = v1 * v2;
    println!("oxygen is {}, co2 is {}, response is {}", v1, v2, r);
}
