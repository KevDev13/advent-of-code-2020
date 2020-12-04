use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn run() {
    let mut total_good: u32 = 0;
    if let Ok(lines) = read_lines("./input/day02.txt") {
        for l in lines {
            if let Ok(to_parse) = l {
                // parse the files here
                let parse_bytes = to_parse.as_bytes();
                let mut start_from = 0;
                // parse everything
                let mut minimum = 0;
                let mut maximum = 0;
                let mut char_to_test = ' ';
                let mut passwd = String::new();
                'begin: for (i, &item) in parse_bytes.iter().enumerate() {
                    if item == b'-' {
                        start_from = i;
                        minimum = match (&to_parse[0..i]).trim().parse::<i32>() {
                            Ok(minimum) => minimum,
                            Err(e) => {
                                println!("error in conversion on minimum");
                                return;
                            }
                        };
                    }
                    if item == b' ' && maximum == 0 {
                        maximum = match (&to_parse[start_from+1..i]).trim().parse::<i32>() {
                            Ok(maximum) => maximum,
                            Err(e) => {
                                println!("error in conversion on maximum");
                                return;
                            }
                        };
                        start_from = i;
                    }
                    if item == b':' {
                        char_to_test = match(&to_parse[i-1..i]).trim().parse::<char>() {
                            Ok(char_to_test) => char_to_test,
                            Err(e) => {
                                println!("error in conversion on char to test");
                                return;
                            }
                        };
                        start_from = i + 1;
                        passwd = to_parse[start_from..].to_string();
                        break 'begin;
                    }
                }
                // puzzle 1 - find the number of characters in the password
                //let mut num_of_chars = 0;
                //for (i, &item) in passwd.as_bytes().iter().enumerate() {
                //    if item as char == char_to_test {
                //        num_of_chars += 1;
                //    }
                //}
                //if num_of_chars >= minimum && num_of_chars <= maximum {
                //    total_good += 1;
                //}
                // puzzle 2
                let passwd_bytes = passwd.as_bytes();
                let char1: char = passwd_bytes[(minimum) as usize] as char;
                let char2: char = passwd_bytes[(maximum) as usize] as char;
                if (char1 == char_to_test && char2 != char_to_test) ||
                    (char2 == char_to_test && char1 != char_to_test) {
                    total_good += 1;
                }
            }
        }
    }
    else {
        println!("File not found");
    }
    println!("{}", total_good);
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
