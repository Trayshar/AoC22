use aoc22::read_aoc_file;

fn main() {
    let mut grid = read_aoc_file(8)
        .map(|line| 
            line.chars().map(|c| 
                c.to_digit(10).expect("Non-numeric character in grid!") as u8
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    let length = grid.len();

    // Check each row and column in both directions for visible trees
    for x in grid.iter_mut() {
        check_visible(x.iter_mut());
        check_visible(x.iter_mut().rev());
    }
    for index in 0..length {
        check_visible(grid.iter_mut().map(|value| &mut value[index]));
        check_visible(grid.iter_mut().map(|value| &mut value[index]).rev());
    }

    // Print out results
    for line in grid.iter() {
        let line: String = line.iter().map(|data| if data & (1 << 7) != 0 {'X'} else {' '} ).collect();
        println!("{}", line);
    }
    let visible: usize = grid.iter().map(|arr| arr.iter().filter(|&&val| val & (1 << 7) != 0).count()).sum();
    println!("Found {} visible trees", visible);

    // ################################## Part 2 #########################################

    let mut highest_score = 0u32;
    for row in 1..length {
        for col in 1..length {
            let val = grid[row][col] & 0b0111_1111;

            let score_up = check_direction(val, (0..row).rev().map(|up| grid[up][col]));
            let score_down = check_direction(val, (row..length).skip(1).map(|down| grid[down][col]));
            let score_right = check_direction(val, (0..col).rev().map(|right| grid[row][right]));
            let score_left = check_direction(val, (col..length).skip(1).map(|left| grid[row][left]));

            let score = score_up * score_down * score_right * score_left;
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    println!("Tree with the highest scenic score is {}", highest_score);
}

/// Checks for visible trees along an axis
fn check_visible<'a, I: Iterator<Item = &'a mut u8>>(mut iter: I) {
    // Always set visibility bit for first item
    let first = iter.next().expect("Empty iterator!");
    *first |= 1 << 7;

    // Keep track of the highest tree on our axis. Remove visibility bit if set.
    let mut highest_tree: u8 = *first & 0b0111_1111;
    while let Some(next) = iter.next() {
        // Remove visibility bit to compare height
        let height = *next & 0b0111_1111;
        if height > highest_tree {
            // println!("Tree {0} is visible, bits: {0:#010b}, heighest: {1}", height, highest_tree);
            *next |= 1 << 7;
            highest_tree = height;
        }
    }
}

/// return the scenic score along the iterator
fn check_direction<I: Iterator<Item = u8>>(val: u8, iter: I) -> u32 {
    let mut score = 0u32;
    for v in iter {
        score += 1;
        // Have to unset visibility bit, grrr
        if v & 0b0111_1111 >= val {
            break;
        }
    }
    score
}