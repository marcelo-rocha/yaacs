
mod bingo;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn parse_comma_list(src: &String)-> Vec<isize> {
    let mut r: Vec<isize> = Vec::new();
    let items = src.strip_suffix("\n").unwrap().split(",");
    for v in items {
        r.push(v.parse().unwrap());
    }
    r
}

fn parse_space_list(src: &String)-> Vec<isize> {
    let mut r: Vec<isize> = Vec::new();
    let items = src.strip_suffix("\n").unwrap().split(" ");
    for v in items {
        let s = v.trim();
        if s.len() > 0 {
            r.push(s.parse().unwrap());
        }
    }
    r
}


fn skip_line(r: &mut BufReader<std::fs::File>) {
    let mut buf = String::new();
    let _ = r.read_line(&mut buf);
}


fn parse_board(r: &mut BufReader<std::fs::File>) -> Option<bingo::BoardNumbers> {

    let mut board: bingo::BoardNumbers = Default::default();

    skip_line(r);
    for y in 0..5 {
        let mut input = String::new();
        let result = r.read_line(&mut input);
        if !result.is_ok() {
            return None
        }
        if input.strip_suffix("\n").is_none(){
            return None
        }
        let values = parse_space_list(&input); 
        for (x, v) in values.into_iter().enumerate() {
            board[y][x] = v as usize;
        }    
    }
    Some(board)
}


fn main() {
    let args_qty = args().len();
    let arg_flag = args().nth(args_qty - 2);
    let show_last_winner = arg_flag.unwrap() == "--last-winner";
    let filename = args().nth(args_qty - 1).unwrap();
    println!("open file {}", filename);

    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut draw_input = String::new();
    let r = reader.read_line(&mut draw_input);
    if r.unwrap() == 0 {
        panic!("unexpected size")
    }
    let draws = parse_comma_list(&draw_input);

    let mut game = bingo::Bingo::new();
    loop {
        let board = parse_board(&mut reader);
        if board.is_none() {
            break
        }
        game.add_board(board.unwrap())
    }

    let mut last_result: Option<isize> = Default::default();
    for (_, v) in draws.into_iter().enumerate() {
        let r = game.draw(v as usize, show_last_winner);
        if r.is_some() {
            last_result = r;
            if !show_last_winner {
                println!("result is {}", r.unwrap());
                return;    
            } 
        }
    }
    if show_last_winner {
        println!("last winner is {}", last_result.unwrap());
        return;
    }
    println!("no winner found");
}   
