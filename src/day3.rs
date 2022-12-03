use aoc22::read_lines;

fn char_to_index(c: char) -> (char, usize) {
    if c.is_ascii_lowercase() {
        return (c, c as usize - ('a' as usize));
    }
    if c.is_ascii_uppercase() {
        return (c, c as usize - ('A' as usize) + 26);
    }

    panic!("Found non-alphabetic character in string!");
}

#[inline]
fn index_to_value(u: usize) -> u32 {
    u as u32 + 1
}

fn main() {
    // Holds a boolean for each item we may encounter
    let mut items: [bool; 52] = [false; 52];
    let mut sum: u32 = 0;

    for line in read_lines("./res/day3.txt").expect("Couldn't read file!") {
        let length = line.chars().count();

        for (_, item) in line.chars().take(length/2).map(char_to_index) {
            items[item] = true;
        }

        for (_, item) in line.chars().skip(length/2).map(char_to_index) {
            if items[item] { // This item has already been found in the other compartment
                sum += index_to_value(item);
                break;
            }
        }

        items = [false; 52];
    }

    println!("The sum of all incorrect items is {}", sum);

    // ################################## Part 2 #########################################
    
    let mut items: [u8; 52] = [0; 52];
    let mut sum: u32 = 0;
    'outer: for (index, line) in read_lines("./res/day3.txt").expect("Couldn't read file!").enumerate() {
        let iter = line.chars().map(char_to_index);
        match index % 3 {
            0 => iter.for_each(|(_, item)| items[item] = 1),
            1 => {
                for (_, item) in iter {
                    if items[item] == 1 {
                        items[item] = 2;
                    }
                };
            },
            2 => {
                for (_, item) in iter {
                    // Found the badge
                    if items[item] == 2 {
                        sum += index_to_value(item);
                        // Reset array and continue outer loop
                        items = [0; 52];
                        continue 'outer;
                    }
                };
            },
            _ => unreachable!()
        };
    }

    println!("The sum of all badges is {}", sum);
}