use aoc22::read_aoc_file;

fn main() {
    let solution = vec![
        vec![2, 1, -3, 3, -2, 0, 4],
        vec![1, -3, 2, 3, -2, 0, 4],
        vec![1, 2, 3, -2, -3, 0, 4],
        vec![1, 2, -2, -3, 0, 3, 4],
        vec![1, 2, -3, 0, 3, 4, -2],
        vec![1, 2, -3, 0, 3, 4, -2],
        vec![1, 2, -3, 4, 0, 3, -2]
    ];
    assert_eq!(3, solve(vec![1, 2, -3, 3, -2, 0, 4], Some(solution)));
    println!("Test finished!");

    let numbers: Vec<i32> = read_aoc_file(20).map(|x| x.parse().unwrap()).collect();
    solve(numbers, None);
}

fn solve(mut numbers: Vec<i32>, solutions: Option<Vec<Vec<i32>>>) -> i32 {
    let mut indices: Box<[usize]> = (0..numbers.len()).collect();

    for time in 0..numbers.len() {
        let index = indices.iter().position(|&i| i == time).unwrap();
        let number = numbers[index];

        if number == 0 {continue;}

        let target = (index as i32 + number - if number < 0 {1} else {0}).rem_euclid(numbers.len() as i32) as usize;

        if index < target {
            // ###I###################Tt##
            // ---[###################]---
            numbers[index..=target].rotate_left(1);
            indices[index..=target].rotate_left(1);

        } else {
            // ###Tt####################I###
            // ----[####################]---
            numbers[target+1..=index].rotate_right(1);
            indices[target+1..=index].rotate_right(1);
        }

        if let Some(ref solution) = solutions { assert_eq!(numbers, solution[time], "T{}: {:?} vs {:?}", time, numbers, solution[time]); }
        print!("\rProcessing index {} from {}", time, numbers.len());
    }
    println!();
    
    let value_0 = numbers.iter().position(|&i| i == 0).unwrap();
    let a = numbers[(1000 + value_0) % numbers.len()];
    let b = numbers[(2000 + value_0) % numbers.len()];
    let c = numbers[(3000 + value_0) % numbers.len()];

    println!("Result: {} = {} + {} + {}", a + b + c, a, b, c);
    a + b + c
}