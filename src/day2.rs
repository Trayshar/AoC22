use aoc22::read_lines;

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

// Indexed as [my_pick][opponent_pick]
const MATCHUPS: [[u32; 3]; 3] = [
    [4, 1, 7],
    [8, 5, 2],
    [3, 9, 6]
];

fn map_input_letters(c: char) -> RPS {
    match c {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => panic!("Unexpected letter!")
    }
}

fn main() {
    let mut score: u32 = 0;
    for line in read_lines("./res/day2.txt").expect("Couldn't read file!") {
        score += line
            .split_ascii_whitespace()
            .map(|s| s.chars().next().unwrap())
            .map(map_input_letters)
            .collect::<Vec<RPS>>()
            .chunks(2)
            .map(|a| MATCHUPS[a[1] as usize][a[0] as usize])
            .next()
            .unwrap();
    }

    println!("The score is {}", score);

    // ################################## Part 2 #########################################

    fn eval(opponent: char, result: char) -> u32 {
        let opponent = map_input_letters(opponent);
        let me = match (opponent, result) {
            (draw, 'Y') => draw,
            (RPS::Rock, 'X') => RPS::Scissors,
            (RPS::Rock, 'Z') => RPS::Paper,
            (RPS::Paper, 'X') => RPS::Rock,
            (RPS::Paper, 'Z') => RPS::Scissors,
            (RPS::Scissors, 'X') => RPS::Paper,
            (RPS::Scissors, 'Z') => RPS::Rock,
            (_ ,_) => panic!("Invalid input")
        };

        MATCHUPS[me as usize][opponent as usize]
    }

    let mut score: u32 = 0;
    for line in read_lines("./res/day2.txt").expect("Couldn't read file!") {
        score += line
            .split_ascii_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|a| eval(a[0], a[1]))
            .next()
            .unwrap();
    }

    println!("Part2 score is {}", score);
}