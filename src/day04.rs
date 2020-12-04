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

    let mut total_valid = 0;
    
    if let Ok(lines) = read_lines("../input/day04.txt") {
        for l in lines {
            if let Ok(parse) = l {
                // if len() == 0, then we are at a blank line and need to check everything we have
                if parse.len() == 0 {
                    //println!("new id");
                    let mut all_good: bool = true;
                    // go through and check to see if all are valid + reset every value to false
                    for c in 0..checks.len() {
                        // if any are false, this will set all_good to false, and will stay there
                        // otherwise, if all are true, all_good will remain true
                        all_good &= checks[c].1;
                        checks[c].1 = false;
                    }
                    total_valid += match all_good {
                        false => 0,
                        true => 1,
                    }
                }
                else {
                    let mut chars = parse.chars();
                    let mut col = 0;
                    while let Some(test_char) = chars.next() {
                        if test_char == ':' as char {
                            let data = &parse[col-3..col];
                            for check in 0..checks.len() {
                                if data == checks[check].0 {
                                    checks[check].1 = true;
                                }
                            }
                        }
                        col += 1;
                    }
                }
            }
        }
        println!("{}", total_valid);
    }
    else {
        println!("error in reading file");
    }
}
