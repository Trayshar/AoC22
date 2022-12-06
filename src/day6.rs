use aoc22::read_lines;

fn main() {
    let line = read_lines("./res/day6.txt").expect("Couldn't read file!").next().expect("Couldn't read file!");

    for (index, bytes) in line.as_bytes().windows(4).enumerate() {
        // Checks for duplicate values in the slice, see https://stackoverflow.com/a/46766782
        if !(1..bytes.len()).any(|i| bytes[i..].contains(&bytes[i - 1])) {
            println!("Found start-of-packet marker at {}", index + 4);
            break;
        }
    }

    // ################################## Part 2 #########################################

    for (index, bytes) in line.as_bytes().windows(14).enumerate() {
        // Checks for duplicate values in the slice, see https://stackoverflow.com/a/46766782
        if !(1..bytes.len()).any(|i| bytes[i..].contains(&bytes[i - 1])) {
            println!("Found start-of-message marker at {}", index + 14);
            break;
        }
    }
}