use aoc22::read_lines;

fn char_to_value(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return c as u32 - ('a' as u32) + 1;
    }
    if c.is_ascii_uppercase() {
        return c as u32 - ('A' as u32) + 27;
    }

    panic!("Found non-alphabetic character in string!");
}

fn main() {
    // Part 1
    let mut sum1: u32 = 0;

    // Part2: Using 3 bitmasks with 64 bits. Each 3 iterations we AND them to get the one item they all have in common
    let mut items: [u64; 3] = [0; 3];
    let mut sum2: u32 = 0;

    for (index, line) in read_lines("./res/day3.txt").expect("Couldn't read file!").enumerate() {
        let length = line.chars().count();

        // Constructing a bitmask where each set bit represents an item.
        let comp1 = line.chars().take(length/2).map(char_to_value).fold(0u64, |acc, value| acc | (1 << value));
        let comp2 = line.chars().skip(length/2).map(char_to_value).fold(0u64, |acc, value| acc | (1 << value));

        // Using AND, we get the item both compartments share
        sum1 += (comp1 & comp2).trailing_zeros();

        // ################################## Part 2 #########################################

        let item = &mut items[index % 3];
        line.chars().map(char_to_value).for_each(|value| *item |= 1 << value);

        if index % 3 == 2 {
            sum2 += items.iter().fold(u64::MAX, |acc, bits| acc & bits).trailing_zeros();
            items = [0; 3];
        }
    }

    println!("The sum of all incorrect items is {}", sum1);
    println!("The sum of all badges is {}", sum2);
}