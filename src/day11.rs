use std::mem;
use aoc22::read_aoc_file;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square()
}

impl Operation {
    fn apply(&self, val: &mut u64) {
        match self {
            Operation::Add(rhs) => *val += rhs,
            Operation::Mul(rhs) => *val *= rhs,
            Operation::Square() => *val *= *val,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    monkey_true: usize,
    monkey_false: usize
}

fn main() {
    let monkeys: Vec<Monkey> = parse_monkeys(read_aoc_file(11).collect::<Vec<_>>());
    
    let monkey_business = play_game(20, monkeys.clone(), |item| *item /= 3);
    println!("[Part 1] Monkey business: {}", monkey_business);

    // Part 2

    // If all 'divisible_by' are co-prime, then we can use the product of them to shrink the item value: https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    let least_common_multiple: u64 = monkeys.iter().map(|monkey| monkey.divisible_by).product();
    let monkey_business = play_game(10000, monkeys, |item| *item %= least_common_multiple);
    println!("[Part 2] Monkey business: {}", monkey_business);
}

fn play_game<F: Fn(&mut u64)>(rounds: u32, mut monkeys: Vec<Monkey>, reduce_worries: F) -> u128 {
    let mut inspections = vec![0u64; monkeys.len()];
    for _round in 1..=rounds {
        for index in 0..monkeys.len() {
            for mut item in mem::take(&mut monkeys[index].items) {
                inspections[index] += 1;
                monkeys[index].operation.apply(&mut item);
                reduce_worries(&mut item);

                let throw_index = if item % monkeys[index].divisible_by == 0 {
                    monkeys[index].monkey_true
                } else {
                    monkeys[index].monkey_false
                };
                monkeys[throw_index].items.push(item);
            }
        }
    }
    let len = inspections.len();
    inspections.sort();
    inspections[len-2] as u128 * inspections[len-1] as u128
}

fn parse_monkeys(lines: Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for chunk in lines.chunks(7) {
        let number: usize = chunk[0]
            .strip_prefix("Monkey ").expect("First line wasn't a monkey...")
            .strip_suffix(":").unwrap()
            .parse().expect("Monkey has no number");
        
        let items: Vec<u64> = chunk[1]
            .strip_prefix("  Starting items: ").expect("Second line wasn't items...")
            .split(", ")
            .map(|item| item.parse().expect("Item isn't a number!"))
            .collect();
        
        let mut operations = chunk[2]
            .strip_prefix("  Operation: new = old ").expect("Second line wasn't operation...")
            .split_ascii_whitespace();
        let operation: Operation = match (operations.next(),operations.next()) {
            (Some("+"), Some("old")) => {
                Operation::Mul(2)
            },
            (Some("+"), Some(rhs)) => {
                Operation::Add(rhs.parse().expect("Operation RHS is not a number!"))
            },
            (Some("*"), Some("old")) => {
                Operation::Square()
            },
            (Some("*"), Some(rhs)) => {
                Operation::Mul(rhs.parse().expect("Operation RHS is not a number!"))
            },
            (_, _) => panic!("Unknown operation!")
        };

        let divisible_by: u64 = chunk[3]
            .strip_prefix("  Test: divisible by ").expect("Third line wasn't divisibility check...")
            .parse().expect("Divisibility check without number!");

        let monkey_true: usize = chunk[4]
            .strip_prefix("    If true: throw to monkey ").expect("Forth line wasn't case true...")
            .parse().expect("Case true without monkey number!");

        let monkey_false: usize = chunk[5]
            .strip_prefix("    If false: throw to monkey ").expect("Fifth line wasn't case false...")
            .parse().expect("Case false without monkey number!");

        assert!(chunk.len() == 6 || chunk[6].is_empty());
        assert!(number == monkeys.len());

        monkeys.push(Monkey{
            items,
            operation,
            divisible_by,
            monkey_true,
            monkey_false        
        })
    }
    monkeys
}