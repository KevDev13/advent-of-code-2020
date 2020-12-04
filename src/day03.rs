use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn run() {
    let mut total: u64 = 1;
    let cols: [usize; 5] = [1, 3, 5, 7, 1];
    let rows: [usize; 5] = [1, 1, 1, 1, 2];
    for c in 0..=4 {
        let mut total_trees = 0;
        let mut row = 0;
        let mut column = 0;
        if let Ok(lines) = read_lines("../input/day03.txt") {
            for l in lines {
                if row % rows[c] == 0 {
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
                        column += cols[c];
                        while column >= parse.len() {
                            column -= parse.len();
                        }
                    }
                }
                row += 1;
            }
            total *= total_trees;
            println!("{}", total_trees);
        }
        else {
            println!("error reading file");
        }
    }
    println!("{}", total);
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
