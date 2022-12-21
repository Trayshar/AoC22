use std::{collections::HashMap, rc::Rc, borrow::Borrow};

use aoc22::{read_aoc_file, Operation};

#[derive(Debug,Clone)]
enum Term {
    Term(Rc<Term>, Operation, Rc<Term>),
    Number(i64),
    Variable()
}

enum Monkey {
    Number(i64),
    Operation(String, Operation, String)
}

fn main() {
    let monkeys: HashMap<String, Monkey> = read_aoc_file(21).map(|line| {
        match line.split_ascii_whitespace().collect::<Box<_>>()[..] {
            [tag, number] => 
                (tag.strip_suffix(":").unwrap().to_owned(), 
                Monkey::Number(number.parse().unwrap())
            ),
            [tag, lhs, operation, rhs] => 
                (tag.strip_suffix(":").unwrap().to_owned(), 
                Monkey::Operation(
                    lhs.to_owned(), 
                    operation.try_into().unwrap(), 
                    rhs.to_owned())
            ),
            _ => panic!()
        }
    }).collect();

    let mut cache: HashMap<&str, Rc<Term>> = HashMap::new();
    let root = get_monkey_number("root", &monkeys, &mut cache);
    let root = match root.borrow() { Term::Number(n) => n, _ => panic!() };
    println!("[Part1] root has value {}", root);


    // Get the two monkeys whose values should match
    let (a, b) = match monkeys.get("root") {
        Some(Monkey::Operation(a, _, b)) => (a.as_str(), b.as_str()),
        _ => panic!()
    };

    // Calculate the values. Set 'humn' to be a variable
    cache.clear();
    cache.insert("humn", Rc::new(Term::Variable()));
    let (lhs, rhs): (Rc<Term>, Rc<Term>) = (get_monkey_number(a, &monkeys, &mut cache), get_monkey_number(b, &monkeys, &mut cache));

    // match both values to a number and a term containing a single 'humn'
    let (mut number, mut term) = match (lhs.borrow(), rhs.borrow()) {
        (Term::Term(_, _, _), Term::Number(num)) => (*num, lhs),
        (Term::Number(num), Term::Term(_, _, _)) => (*num, rhs),
        (Term::Number(num), Term::Variable()) => (*num, rhs),
        (Term::Variable(), Term::Number(num)) => (*num, lhs),
        _ => panic!(),
    };

    // (lhs <op> rhs) = res
    while let Term::Term(lhs, operation, rhs) = term.borrow() {
        let inv = operation.inverse();
        match (lhs.borrow(), rhs.borrow()) {
            // (term <op> num) = res
            (_, Term::Number(rhs)) => {
                // term = res <inv> num
                number = inv.apply(&number, rhs);
                term = lhs.clone();
            },
            // (num <op> term) = res
            (Term::Number(lhs), _) => {
                // (num <op> term) = (term <op> num) = res
                if operation.isCommutative() {
                    // term = res <inv> num
                    number = inv.apply(&number, lhs);
                    term = rhs.clone();
                } else {
                    // num = res <inv> term
                    let tmp = *lhs;
                    // SAFETY: This will never cause a loop, because the inverse operation is commutative.
                    term = Rc::new(Term::Term(Rc::new(Term::Number(number)), inv, rhs.clone()));
                    number = tmp;
                }
            },
            _ => panic!()
        };
    }

    println!("[Part2] The value of 'humn' is {}", number);
}

fn get_monkey_number<'a>(tag: &'a str, monkeys: &'a HashMap<String, Monkey>, cache: &mut HashMap<&'a str, Rc<Term>>) -> Rc<Term> {
    if let Some(term) = cache.get(tag) {
        return term.clone();
    }

    let monkey = monkeys.get(tag).expect("monkey should be known");
    let result = match monkey {
        Monkey::Number(num) => {
            Term::Number(*num)
        },
        Monkey::Operation(lhs, operation, rhs) => {
            let lhs = get_monkey_number(lhs, monkeys, cache);
            let rhs = get_monkey_number(rhs, monkeys, cache);

            match (lhs.borrow(), rhs.borrow()) {
                (Term::Number(lhs), Term::Number(rhs)) => {
                    Term::Number(operation.apply(lhs, rhs))
                },
                (_, _) => Term::Term(lhs, *operation, rhs)
            }
        }
    };

    let result = Rc::new(result);
    cache.insert(tag, result.clone());
    result
}