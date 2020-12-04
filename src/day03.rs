use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn run() {
    let mut total_trees: u32 = 0;
    let mut column = 0;
    if let Ok(lines) = read_lines("./input/day03.txt") {
        for l in lines {
            if let Ok(parse) = l {
                let test_char = match (&parse[column..column+1]).trim().parse::<char>() {
                    Ok(test_char) => test_char,
                    Err(_e) => {
                        println!("error in getting character to test");
                        return;
                    }
                };
                total_trees += match test_char {
                    '#' => 1,
                    _ => 0,
                };
                column += 3;
                while column >= parse.len() {
                    column -= parse.len();
                }
            }
        }
        println!("{}", total_trees);
    }
    else {
        println!("error reading file");
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
