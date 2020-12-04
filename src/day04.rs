use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_byr(s: String) -> bool {
    true
}

fn check_iyr(s: String) -> bool {
    true
}

fn check_eyr(s: String) -> bool {
    true
}

fn check_hgt(s: String) -> bool {
    true
}

fn check_hcl(s: String) -> bool {
    true
}

fn check_ecl(s: String) -> bool {
    true
}

fn check_pid(s: String) -> bool {
    true
}

pub fn run() {
    let mut checks = [("byr", false, false, check_byr as fn(String) -> bool),
                      ("iyr", false, false, check_iyr as fn(String) -> bool),
                      ("eyr", false, false, check_eyr as fn(String) -> bool),
                      ("hgt", false, false, check_hgt as fn(String) -> bool),
                      ("hcl", false, false, check_hcl as fn(String) -> bool),
                      ("ecl", false, false, check_ecl as fn(String) -> bool),
                      ("pid", false, false, check_pid as fn(String) -> bool)];

    let mut total_valid = 0;
    
    if let Ok(lines) = read_lines("../input/day04.txt") {
        for l in lines {
            if let Ok(parse) = l {
                // if len() == 0, then we are at a blank line and need to check everything we have
                // note, this method assumes a blank line at the end of the input file
                // otherwise, we will not check the final entry
                if parse.len() == 0 {
                    //println!("new id");
                    let mut all_good: bool = true;
                    // go through and check to see if all are valid + reset every value to false
                    for c in 0..checks.len() {
                        // if any are false, this will set all_good to false, and will stay there
                        // otherwise, if all are true, all_good will remain true
                        all_good &= checks[c].1 && checks[c].2;
                        checks[c].1 = false;
                        checks[c].2 = false;
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
                            // find which piece of data this is and set that flag to true
                            for check in 0..checks.len() {
                                if data == checks[check].0 {
                                    checks[check].1 = true;
                                    //checks[check].2 = checks[check].3(data.to_string());
                                }
                            }
                            // validate the data is correct
                            
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
