use aoc22::read_lines;
use aoc22::contains_duplicates;

fn main() {
    let line = read_lines("./res/day6.txt").expect("Couldn't read file!").next().expect("Couldn't read file!");

    for (index, bytes) in line.as_bytes().windows(4).enumerate() {
        if !contains_duplicates(bytes) {
            println!("Found start-of-packet marker at {}", index + 4);
            break;
        }
    }

    // ################################## Part 2 #########################################

    for (index, bytes) in line.as_bytes().windows(14).enumerate() {
        if !contains_duplicates(bytes) {
            println!("Found start-of-message marker at {}", index + 14);
            break;
        }
    }
}