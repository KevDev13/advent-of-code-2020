use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn run() {
    let mut total_bad: u32 = 0;
    if let Ok(lines) = read_lines("day02.txt") {
        for l in lines {
            if let Ok(to_parse) = l {
                // parse the files here
                println!("{}", to_parse);
            }
        }
    }
    else {
        println!("File not found");
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
