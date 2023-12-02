use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static PATH: &str = "data/input.txt";

fn main() -> Result<(), io::Error> {
    let lines = read_lines(PATH)?;

    let result: u32 = lines.map(|l| decode2(l.unwrap())).sum();
    println!("\n{}", result);

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn id_if_possible(s: String) -> Option<u32> {



    let first_digit = get_first_digit(s.as_str());
    let last_digit = get_last_digit(s.as_str());

    let result = digits_to_number(first_digit, last_digit);

    println!("{result}");
    result
}

/// Get the maximum cubes required for each color (RGB)
fn parse_game(s: &str) -> (u32, u32, u32) {

}

/// Get the cubes shown in a single showing (RGB)
fn parse_game(s: &str) -> (u32, u32, u32) {
    
}
