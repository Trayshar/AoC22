use std::collections::{BTreeSet};

use aoc22::read_aoc_file;

type Position = (i32, i32);
fn main() {
    let instructions: Vec<_> = read_aoc_file(9).map(|line| {
        let mut line = line.split_ascii_whitespace();
        let direction = line.next().unwrap().chars().next().unwrap();
        let steps: u32 = line.next().unwrap().parse().unwrap();

        let direction: Position = match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("Undefined direction \"{}\"", direction)
        };
        (steps, direction)
    }).collect();

    let part1 = rope_simulation(2, instructions.iter());
    println!("Part1: Tail visited {} positions!", part1);

    let part2 = rope_simulation(10, instructions.iter());
    println!("Part2: Tail visited {} positions!", part2);
}

fn rope_simulation<'a, I: Iterator<Item=&'a (u32, Position)>>(knot_count: usize, instructions: I) -> usize {
    // All positions visited by the tail. It's a set, so no duplicates.
    let mut positions = BTreeSet::<Position>::new();
    // Positions of each knot
    let mut knots = vec![(0, 0); knot_count];

    // Mark starting position as visited
    positions.insert((0, 0));

    for (steps, direction) in instructions {
        for _ in 0..*steps {
            let head = &mut knots[0];
            head.0 += direction.0; head.1 += direction.1;
            
            // Iterate all knots in pairs of 2. The pairs overlap.
            for index in 1..knot_count {
                if let [prev, current] = &mut knots[index-1..=index] {
                    let x_diff = prev.0 - current.0;
                    let y_diff = prev.1 - current.1;
                    if x_diff.abs() > 1 || y_diff.abs() > 1 {
                        current.0 += x_diff.signum();
                        current.1 += y_diff.signum();
                    }
                } else {
                    // SAFETY: The slice has a size of two and is valid; Therefor, the deconstruction cannot fail.
                    unreachable!();
                }
            }
            // After each step, mark the tail's position as marked
            positions.insert(*knots.last().unwrap());
        }
    }

    // Return the number of visited positions
    positions.len()
}