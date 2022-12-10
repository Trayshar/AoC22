use aoc22::read_aoc_file;

fn main() {
    let instructions = read_aoc_file(10).map(|instruction| {
        let mut instruction = instruction.split_ascii_whitespace();
        match instruction.next() {
            Some("noop") => ("noop", 0),
            Some("addx") => ("addx", instruction.next().expect("addx without number!").parse().expect("addx without number!")),
            None => panic!("Empty line!"),
            cmd => panic!("Undefined command \"{}\"", cmd.unwrap())
        }
    });

    let mut cycle: i32 = 1;
    let mut register: i32 = 1;
    let mut sum: i32 = 0;
    // part2
    let mut crt = [false; 240];

    for instruction in instructions {
        tick_cycle(&cycle, &register, &mut sum, &mut crt);
        match instruction {
            ("noop", _) => {
                cycle += 1;
            },
            ("addx", addx) => {
                cycle += 1;
                tick_cycle(&cycle, &register, &mut sum, &mut crt);
                cycle += 1;
                register += addx;
            },
            _ => unreachable!()
        };
    }
    println!("The sum of the six signal strengths is {}", sum);
    // Part 2
    for chunk in crt.chunks(40) {
        let mut line = String::new();
        for &vis in chunk {
            line.push(if vis {'#'} else {'.'});
        }
        println!("{}", line)
    }
}

fn tick_cycle(cycle: &i32, register: &i32, sum: &mut i32, crt: &mut [bool]) {
    if *cycle >= 20 && (cycle - 20) % 40 == 0 {
        *sum += register * *cycle;
    }

    // Part 2
    let index = cycle - 1;
    assert!(index >= 0);
    if ((register-1)..=(register+1)).contains(&(index % 40)) {
        crt[index as usize] = true;
    }
}