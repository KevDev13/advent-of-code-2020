use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run() {
    let mut max = 0;
    if let Ok(lines) = read_lines("./input/day05.txt") {
        for line in lines {
            if let Ok(parse) = line {
                let mut row: u8 = 0;
                let mut seat: u8 = 0;
                let seat_id: u32;

                // find row
                let row_string = &parse[..=6];
                let mut row_chars = row_string.chars();
                let mut bit = 6;
                while let Some(fb) = row_chars.next() {
                    let new_bit = match fb {
                        'F' => 0,
                        'B' => 1,
                        _ => {
                            println!("error in matching FB");
                            return;
                        }
                    };
                    row = row | new_bit << bit;
                    bit -= 1;
                }

                // find seat
                let seat_string = &parse[7..];
                let mut seat_chars = seat_string.chars();
                bit = 2;
                while let Some(lr) = seat_chars.next() {
                    let new_bit = match lr {
                        'L' => 0,
                        'R' => 1,
                        _ => {
                            println!("error in matching LR");
                            return;
                        }
                    };
                    seat = seat | new_bit << bit;
                    bit -= 1;
                }

                seat_id = row as u32 * 8 + seat as u32;

                // check if seat id is greater than the current max
                if seat_id > max {
                    max = seat_id;
                }
            }
        }
        println!("{}", max);
    }
    else {
        println!("error reading file");
    }
}
