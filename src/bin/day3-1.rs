use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static PATH: &str = "data/day3-small.txt";

fn main() {
    let lines = read_lines(PATH).map(|x| x.unwrap());

    let (numbers, symbols) = parse_input(lines);

    // println!("{:?}", numbers);
    // println!("{:?}", symbols);

    let result: u32 = calc_result(numbers, symbols);
    println!("{result}");
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn parse_input(
    lines: impl Iterator<Item = String>,
) -> (
    HashMap<usize, HashMap<u32, (usize, usize)>>,
    HashMap<usize, Vec<usize>>,
) {
    let mut numbers: HashMap<usize, HashMap<u32, (usize, usize)>> = HashMap::new();
    let mut symbols: HashMap<usize, Vec<usize>> = HashMap::new();

    let re_number = Regex::new(r"(\d+)").unwrap();
    let re_symbol = Regex::new(r"(\D)").unwrap();

    let mut line_nr = 0usize;
    for line in lines {
        let number_matches = re_number.find_iter(line.as_str());
        for m in number_matches {
            let number = m.as_str();
            let start = m.start();
            let end = start + number.len() - 1;
            let number = number.parse().unwrap();

            match numbers.get_mut(&line_nr) {
                Some(map) => {
                    map.insert(number, (start, end));
                }
                None => {
                    let mut map = HashMap::new();
                    map.insert(number, (start, end));
                    numbers.insert(line_nr, map);
                }
            }
        }

        let symbol_matches = re_symbol.find_iter(line.as_str());
        for m in symbol_matches.filter(|m| m.as_str() != ".") {
            let index = m.start();

            match symbols.get_mut(&line_nr) {
                Some(vec) => {
                    vec.push(index);
                }
                None => {
                    let vec = vec![index];
                    symbols.insert(line_nr, vec);
                }
            }
        }

        line_nr += 1;
    }

    (numbers, symbols)
}

fn calc_result(
    numbers: HashMap<usize, HashMap<u32, (usize, usize)>>,
    symbols: HashMap<usize, Vec<usize>>,
) -> u32 {
    let mut result = 0;
    for (line_nr, numbers) in numbers.iter() {
        let mut relevant_symbols = HashSet::new();
        let min_line = match line_nr {
            0 => 0,
            _ => line_nr - 1,
        };
        for i in min_line..(line_nr + 2) {
            let line_symbols = symbols.get(&i);
            if let Some(s) = line_symbols {
                relevant_symbols.extend(s);
            }
        }

        for (n, (start, end)) in numbers.iter() {
            println!("\n{n} - line {line_nr} - indices {start}-{end}");
            println!("symbol indices: {:?}", relevant_symbols);

            if number_is_relevant(*start, *end, &relevant_symbols) {
                println!("YES");
                result += n;
            } else {
                println!("NO");
            };
        }
    }
    result
}

fn number_is_relevant(start: usize, end: usize, symbol_indices: &HashSet<usize>) -> bool {
    let start = match start {
        0 => 0,
        _ => start - 1,
    };
    let indices_set: HashSet<usize> = (start..(end + 2)).collect();
    println!("number indices: {:?}", indices_set);
    !indices_set.is_disjoint(symbol_indices)
}
