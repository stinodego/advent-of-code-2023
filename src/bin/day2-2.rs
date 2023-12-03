use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static PATH: &str = "data/day2.txt";

fn main() {
    let lines = read_lines(PATH).map(|x| x.unwrap());
    let result: u32 = lines.map(power).sum();
    println!("{result}");
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn power(s: String) -> u32 {
    let game_string = s.split(": ").nth(1).unwrap();
    let game = parse_game(game_string);
    determine_game_power(game)
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

fn determine_game_power<'a>(
    game: impl Iterator<Item = impl Iterator<Item = (u32, &'a str)>>,
) -> u32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for showing in game {
        for color in showing {
            let n = color.0;
            let color = color.1;
            match color {
                "red" => red = std::cmp::max(red, n),
                "green" => green = std::cmp::max(green, n),
                "blue" => blue = std::cmp::max(blue, n),
                _ => unreachable!(),
            };
        }
    }
    red * green * blue
}
