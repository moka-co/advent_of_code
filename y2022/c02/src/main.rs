
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;

//Copied this from someone on reddit, sorry!

/*
Take:
A = rock
B = paper
C = scissor

Response:
X = rock -> 1
Y = paper -> 2
Z = scissor -> 3

X > C -> 1 > 3
Y > A -> 2 > 1
Z > B -> 3 > 1
*/

fn parse_move(c: u8) -> i32 {
    match c {
        b'A' => 1,
        b'B' => 2,
        b'C' => 3,
        b'X' => 1,
        b'Y' => 2,
        b'Z' => 3,
        _ => unimplemented!(),
    }
}


fn main() -> io::Result<()> {
    let filepath = String::from("");
    let f = File::open(filepath)?;
    let f = BufReader::new(f);
    let mut moves:Vec<(i32, i32)> = vec![];


    for line in f.lines() {
        let line = line.unwrap().into_bytes();
        let (oponent, you) = (parse_move(line[0]), parse_move(line[2]));
        moves.push( (oponent, you) );
    }

    let mut total_score : i32 = 0;
    for (op, you) in &moves {
        let outcome = match (op, you) {
        (1, 1) => 3, //Tie
        (2, 2) => 3, //Tie
        (3, 3) => 3, //Tie

        (1, 2) => 6, //we win
        (2, 3) => 6, //we win
        (3, 1) => 6, //we win
        
        (2, 1) => 0, //they win
        (3, 2) => 0, //they win
        (1, 3) => 0, //they win
        _ => unreachable!()
    };
    total_score += you + outcome;
    }
    println!{"total score: {}",total_score};

    //2nd part
    let mut total_score = 0;
    for (op, outcome) in &moves {
        let ourmove = match (outcome, op) {
            // We lose
            (1, 1) => 3,
            (1, 2) => 1,
            (1, 3) => 2,
            // We tie
            (2, 1) => 1,
            (2, 2) => 2,
            (2, 3) => 3,
            // We win
            (3, 3) => 1,
            (3, 1) => 2,
            (3, 2) => 3,
            _ => unreachable!()
        };
        let outcome_score = (outcome - 1) * 3;
        total_score += outcome_score + ourmove;
    }
    println!("total score 2: {}", total_score);

    Ok(())
}
