#[path = "./day06.rs"] mod day06;

pub fn run(){
    let mut commands = Vec::new();
    let lines = day06::read_lines("./input/day08.txt")
        .expect("error reading file");
    for line in lines {
        let parse = line.expect("error parsing line");
        let cmd = String::from(&parse[0..=2]);
        let pos_or_neg = &parse[4..5];
        let mut number = (&parse[5..]).trim().parse::<isize>()
            .expect("error parsing number");
        let mut tuple = (cmd, 0, false);
        if pos_or_neg == "-" {
            number = 0 - number;
        }
        tuple.1 = number;
        commands.push(tuple);
    }

    let mut c: isize = 0;
    let mut acc = 0;
    loop {
        if commands[c as usize].2 {
            break;
        }
        commands[c as usize].2 = true;
        match commands[c as usize].0.as_str() {
            "nop" => c += 1,
            "acc" => {
                acc += commands[c as usize].1;
                c += 1;
            }
            "jmp" => c += commands[c as usize].1,
            _ => {
                println!("error in matching command");
                return;
            }
        }
    }

    println!("{}", acc);
}
