use aoc22::read_lines;

fn main() {
    let mut cargo: Vec<Vec<char>> = Vec::new();

    for line in read_lines("./res/day5.txt").expect("Couldn't read file!") {
        if line.contains("[") {
            for (index, crate_str) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
                let stack = match cargo.get_mut(index) {
                    Some(s) => s,
                    None => {
                        cargo.push(Vec::new());
                        cargo.get_mut(index).unwrap()
                    }
                };
                if crate_str.contains(&'[') {
                    // crate_str looks like "[X] "
                    stack.push(*crate_str.get(1).unwrap());
                }
            }
        } else if line.starts_with("move") {
            let numbers: Vec<u32> = line.split_ascii_whitespace().filter_map(|token| token.parse::<u32>().ok()).collect();
            assert_eq!(numbers.len(), 3);
            let (quantity, from, to) = (numbers[0], numbers[1] - 1, numbers[2] - 1);

            let from = &mut cargo[from as usize];
            let mut crates = from.split_off(from.len() - quantity as usize);
            // (Un)comment this line to toggle between part 1 or 2
            // crates.reverse();

            let to = &mut cargo[to as usize];
            to.append(&mut crates);
        } else if line.is_empty() {
            println!("{:?}", cargo);
            cargo.iter_mut().for_each(|x| x.reverse());
            println!("{:?}", cargo);
        }
    }
    println!("{:?}", cargo);
    let res: String = cargo.iter().map(|stack| stack.last().unwrap()).collect();
    println!("{}", res);
}