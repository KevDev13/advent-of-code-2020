use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_byr(s: String) -> bool {
    if s.len() < 4 {
        return false;
    }
    let year = match (&s[0..4]).trim().parse::<i32>() {
        Ok(year) => year,
        Err(_e) => {
            return false;
        }
    };
    
    if year >= 1920 && year <= 2002 {
        return true;
    }

    false
}

fn check_iyr(s: String) -> bool {
    if s.len() < 4 {
        return false;
    }
    
    let year = match (&s[0..4]).trim().parse::<i32>() {
        Ok(year) => year,
        Err(_e) => {
            return false;
        }
    };

    if year >= 2010 && year <= 2020 {
        return true;
    }
    
    false
}

fn check_eyr(s: String) -> bool {
    if s.len() < 4 {
        return false;
    }
    
    let year = match (&s[0..4]).trim().parse::<i32>() {
        Ok(year) => year,
        Err(_e) => return false,
    };

    if year >= 20210 && year <= 2030 {
        return true;
    }
    
    false
}

fn check_hgt(s: String) -> bool {
    if s.len() < 4 {
        return false;
    }

    
    
    true
}

fn check_hcl(s: String) -> bool {
    if s.len() < 7 {
        return false;
    }

    let hash = match (&s[0..1]).trim().parse::<char>() {
        Ok(hash) => hash,
        Err(_e) => return false,
    };

    if hash != '#' {
        return false;
    }
    
    let alpha = &s[1..7];
    let mut alpha_chars = alpha.chars();
    while let Some(test_char) = alpha_chars.next() {
        match test_char {
            'a' => (), 'b' => (), 'c' => (), 'd' => (), 'e' => (), 'f' => (),
            '1' => (), '2' => (), '3' => (), '4' => (), '5' => (),
            '6' => (), '7' => (), '8' => (), '9' => (), '0' => (),
            _ => return false,
        }
    }
    true
}

fn check_ecl(s: String) -> bool {
    if s.len() < 3 {
        return false;
    }

    let color = &s[0..3];
    match color {
        "amb" => return true,
        "blu" => return true,
        "brn" => return true,
        "gry" => return true,
        "grn" => return true,
        "hzl" => return true,
        "oth" => return true,
        _ => return false,
    }
}

fn check_pid(s: String) -> bool {
    if s.len() < 9 {
        return false;
    }

    let _number = match (&s[0..9]).trim().parse::<i32>() {
        Ok(number) => number,
        Err(_e) => return false,
    };

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
    
    if let Ok(lines) = read_lines("./input/day04.txt") {
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
                                    let rest_of_line = &parse[col..];
                                    checks[check].2 = checks[check].3(rest_of_line.to_string());
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
