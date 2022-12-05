// Advent of Code 2022 Number Two
// Code by the ThePrimeTime

use anyhow::Result;
use std::str::FromStr;

#[derive(Debug)]
struct HandPair1 {
    value: usize,
}


const WIN_LOSE: [usize; 3] = [3, 6, 0];

fn to_number(c: &str) -> usize {
    return match c {
        "A" => 0,
        "B" => 2,
        "C" => 1,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!("not very cool input")
    }
}


impl FromStr for HandPair1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (o, p) = match s.split_once(" ") {
            Some((o, p)) => (o, p), 
            None => return Err(anyhow::anyhow!("invalid input")),
        };

        let o2 = to_number(o);
        let p2 = to_number(p);
        let score = p2 + WIN_LOSE[(2 + o2 + p2) % WIN_LOSE.len()];

        return Ok(HandPair1 { value: score });
    }
}
        

fn main() -> Result<()> {
    let values: usize = include_str!("../input.txt")
        .lines()
        .flat_map(|x| x.parse::<HandPair1>())
        .map(|x| x.value)
        .sum();
    
    println!("Result: {:?}", values);
    return Ok(());
        
}


    // AX = Rock (1)
    // BY = Paper (2) 
    // CZ = Scissors (3)
    //
    // WIN = 6
    // LOSS = 0
    // DRAW = 3
