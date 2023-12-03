use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static PATH: &str = "data/day2.txt";
static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;

fn main() -> Result<(), io::Error> {
    let lines = read_lines(PATH)?;

    let result: u32 = lines.filter_map(|l| id_if_possible(l.unwrap())).sum();
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
    let mut split = s.split(": ");
    let id_string = split.next().unwrap();
    let game_string = split.next().unwrap();

    let id = parse_id(id_string);
    let game = parse_game(game_string);
    let possible = determine_game_possibility(game);

    if possible {
        Some(id)
    } else {
        None
    }
}

/// Parse the game ID from the game ID string
fn parse_id(s: &str) -> u32 {
    s.split_whitespace().nth(1).unwrap().parse().unwrap()
}
fn parse_game(s: &str) -> impl Iterator<Item = impl Iterator<Item = (u32, &str)>> {
    s.split(';').map(parse_showing)
}
fn parse_showing(s: &str) -> impl Iterator<Item = (u32, &str)> {
    s.split(',').map(parse_color)
}
fn parse_color(s: &str) -> (u32, &str) {
    let mut split = s.split_whitespace();
    let n = split.next().unwrap().parse().unwrap();
    let color = split.next().unwrap();
    (n, color)
}

fn determine_game_possibility<'a>(
    mut game: impl Iterator<Item = impl Iterator<Item = (u32, &'a str)>>,
) -> bool {
    game.all(determine_showing_possibility)
}
fn determine_showing_possibility<'a>(mut showing: impl Iterator<Item = (u32, &'a str)>) -> bool {
    showing.all(determine_color_possibility)
}
fn determine_color_possibility(color: (u32, &str)) -> bool {
    let n = color.0;
    let color = color.1;

    let max = match color {
        "red" => MAX_RED,
        "green" => MAX_GREEN,
        "blue" => MAX_BLUE,
        _ => unreachable!(),
    };
    n <= max
}
