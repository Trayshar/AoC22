use aoc22::read_lines;

fn main() {
    let res: (u32, u32) = read_lines("./res/day4.txt")
        .expect("Couldn't read file!")
        .map(|s| {
            let mut r = s.split(",").map(|p| {
                let mut p = p.split("-").map(str::parse::<u32>).map(Result::unwrap);
                (p.next().unwrap(), p.next().unwrap())
            });
            (r.next().unwrap(), r.next().unwrap())
        })
        .fold((0, 0), |(overlap, contained), ((s1, e1), (s2, e2))| {
            let overlap = overlap + (s1 <= e2 && e1 >= s2) as u32;
            let contained = contained + ((s1 <= s2 && e1 >= e2) || (s1 >= s2 && e1 <= e2)) as u32;
            (overlap, contained)
        });

    println!("There are {} contained pairs!", res.1);

    // ################################## Part 2 #########################################

    println!("There are {} overlapping pairs!", res.0);
}