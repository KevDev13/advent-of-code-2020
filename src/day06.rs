use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run() {
    let mut sum = 0;
    let mut letters = [("a", false), ("b", false), ("c", false), ("d", false),
                       ("e", false), ("f", false), ("g", false), ("h", false),
                       ("i", false), ("j", false), ("k", false), ("l", false),
                       ("m", false), ("n", false), ("o", false), ("p", false),
                       ("q", false), ("r", false), ("s", false), ("t", false),
                       ("u", false), ("v", false), ("w", false), ("x", false),
                       ("y", false), ("z", false)];
    if let Ok(lines) = read_lines("./input/day06.txt") {
        for line in lines {
            if let Ok(parse) = line {
                if parse.len() == 0 {
                    for c in 0..letters.len() {
                        sum += match letters[c].1 {
                            true => 1,
                            false => 0,
                        };
                        letters[c].1 = false;
                    }
                }
                else {
                    for c in 0..parse.len() {
                        let character = &parse[c..c+1];
                        for d in 0..letters.len() {
                            if character == letters[d].0 {
                                letters[d].1 = true;
                            }
                        }
                    }
                }
            }
        }
        println!("{}", sum);
    }
    else {
        println!("unable to open file");
    }
}
