use std::io::{self, prelude::*};
use std::error;
use std::fmt;

fn main() {
    
}

enum Tile {
    Water,
    Sand,
    Clay,
}


fn read_input() -> io::Result<Vec<Vec<Tile>>> {
    for line in io::stdin().lock().lines() {
        let line = line?;
        // we want to break the line into two parts the x and y, then return a value
        // based on that
        let value = match parse_line(&line) {

        }
    }

    Ok(vec![vec![]])
}

enum RangeOrVal {
    Range(Vec<usize>),
    Val(usize)
}

// returns the x range or value then the y range or val
fn parse_line(string: &String) -> (RangeOrVal, RangeOrVal) {
    let mut x = String::new();
    let mut y = String::new();
    // extract the range or val and write it in as a string to x and y
    for charr in string.chars() {
        // find the x or y and then start setting the 
    }

    // then convert x and y into the enum
}