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
    let mut num_of_people = 0;
    let mut letters = [("a", 0), ("b", 0), ("c", 0), ("d", 0),
                       ("e", 0), ("f", 0), ("g", 0), ("h", 0),
                       ("i", 0), ("j", 0), ("k", 0), ("l", 0),
                       ("m", 0), ("n", 0), ("o", 0), ("p", 0),
                       ("q", 0), ("r", 0), ("s", 0), ("t", 0),
                       ("u", 0), ("v", 0), ("w", 0), ("x", 0),
                       ("y", 0), ("z", 0)];
    if let Ok(lines) = read_lines("./input/day06.txt") {
        for line in lines {
            if let Ok(parse) = line {
                if parse.len() == 0 {
                    for c in 0..letters.len() {
                        if letters[c].1 == num_of_people {
                            sum += 1;
                        }
                        letters[c].1 = 0;
                    }
                    num_of_people = 0;
                }
                else {
                    for c in 0..parse.len() {
                        let character = &parse[c..c+1];
                        for d in 0..letters.len() {
                            if character == letters[d].0 {
                                letters[d].1 += 1;
                            }
                        }
                    }
                    num_of_people += 1;
                }
            }
        }
        println!("{}", sum);
    }
    else {
        println!("unable to open file");
    }
}
