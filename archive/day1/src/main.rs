use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static PATH: &str = "data/input.txt";
static DIGITS: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
    HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
});

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

fn decode2(s: String) -> u32 {
    let first_digit = get_first_digit(s.as_str());
    let last_digit = get_last_digit(s.as_str());

    let result = digits_to_number(first_digit, last_digit);

    println!("{result}");
    result
}

fn get_first_digit(s: &str) -> u32 {
    for start in 0..s.len() {
        let substring = &s[start..];
        let digit = get_digit_at_start(substring);
        if digit.is_some() {
            return get_digit_at_start(substring).unwrap();
        }
    }
    0 // sentinel value
}

fn get_last_digit(s: &str) -> u32 {
    for start in (0..s.len()).rev() {
        let substring = &s[start..];
        let digit = get_digit_at_start(substring);
        if digit.is_some() {
            return get_digit_at_start(substring).unwrap();
        }
    }
    0 // sentinel value
}

fn get_digit_at_start(s: &str) -> Option<u32> {
    if let Some(digit) = s.chars().next().unwrap().to_digit(10) {
        return Some(digit);
    }
    for (word, digit) in DIGITS.iter() {
        if s.starts_with(word) {
            return Some(*digit);
        }
    }
    None
}

// fn decode(s: String) -> u32 {
//     println!("\n{}", s);

//     let re_digit = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
//     let replacement = |caps: &Captures| DIGITS.get(&caps[0]).unwrap();
//     let s = re_digit.replace_all(s.as_str(), replacement).to_string();

//     // for (string, digit) in DIGITS.iter() {
//     //     s = s.replace(string, digit);
//     // }
//     println!("{}", s);

//     let first_digit = s.chars().filter_map(|c| c.to_digit(10)).next().unwrap();
//     let last_digit = s
//         .chars()
//         .rev()
//         .filter_map(|c| c.to_digit(10))
//         .next()
//         .unwrap();

//     println!("{first_digit} {last_digit}");

//     let result = digits_to_number(first_digit, last_digit);

//     println!("{result}");
//     result
// }

fn digits_to_number(first: u32, last: u32) -> u32 {
    first * 10 + last
}
