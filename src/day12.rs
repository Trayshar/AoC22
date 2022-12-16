use aoc22::read_aoc_file;

fn main() {
    let grid: Vec<Vec<u8>> = read_aoc_file(12).map(|line| line.chars().map(|c| c as u8 - b'a').collect()).collect();
    let start: (usize, usize) = todo!();
    let goal: (usize, usize) = todo!();
    let len = grid.len();

    let dijkstra = vec![vec![u8::MAX; len]; len];

}