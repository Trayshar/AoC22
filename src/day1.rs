use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut cal: u32 = 0;
    let mut elves: Vec<u32> = Vec::new();

    for line in read_lines("./res/day1.txt").expect("Couldn't open file!") {
        let line = line.expect("Couldn't read lines!");

        if line.is_empty() {
            // Empty line found, pushing added calories onto vector
            elves.push(cal);
            // Reset current calories count
            cal = 0;
        } else {
            // Found another snack, add to current calories
            cal += line.parse::<u32>().expect("Line is not a number?!");
        }
    }

    // Last elf didn't have a blank line afterwards, so we have to manually push his calories
    elves.push(cal);

    let max = *elves.iter().max().unwrap();
    println!("The elf with the most calories carries \"{}\" calories", max);

    // ################################## Part 2 #########################################

    // elves has 250 items, so this isn't expensive.
    elves.sort();
    // Get a reference to the last 3 elements in the vector which is sorted in ascending order. 
    // These are our top 3 calories-carrying elves
    let t3 = &elves[(elves.len()-3)..];

    println!("The 3 elves with the most calories carry \"{}\" in total", t3.iter().sum::<u32>());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
//
// See https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}