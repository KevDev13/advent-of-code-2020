use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run() {
    let mut checks = [("byr", false), ("iyr", false), ("eyr", false), ("hgt", false),
                      ("hcl", false), ("ecl", false), ("pid", false)];
    const BYR: u8 = 0;
    const IYR: u8 = 1;
    const EYR: u8 = 2;
    const HGT: u8 = 3;
    const HCL: u8 = 4;
    const ECL: u8 = 5;
    const PID: u8 = 6;

    let mut total_valid = 0;
    
    if let Ok(lines) = read_lines("../input/day04.txt") {
        for l in lines {
            if let Ok(parse) = l {
                // if len() == 0, then we are at a blank line and need to check everything we have
                if parse.len() == 0 {
                    let mut all_good: bool = true;
                    // go through and check to see if all are valid + reset every value to false
                    for c in 0..checks.len() {
                        // if any are false, this will set all_good to false, and will stay there
                        // otherwise, if all are true, all_good will remain true
                        all_good = all_good && checks[c].1;
                        checks[c].1 = false;
                    }
                    total_valid += match all_good {
                        false => 0,
                        true => 1,
                    }
                }
                else {
                    
                }
            }
        }
    }
    else {
        println!("error in reading file");
    }
}
