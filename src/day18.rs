use std::collections::{HashSet, VecDeque};
use aoc22::read_aoc_file;

type Cube = (i8, i8, i8);

fn main() { 
    let lava: HashSet<Cube> = read_aoc_file(18).map(|line| {
        let coords: Box<[_]> = line.split(",").map(|c| c.parse().unwrap()).collect();
        assert!(coords.len() == 3);
        (coords[0], coords[1], coords[2])
    }).collect();

    let surface: usize = lava.iter().map(|cube| {
        adjacent_cubes(cube).filter(|adj| !lava.contains(adj)).count()
    }).sum();

    println!("[Part1] Total surface area is {}", surface);

    let min_x = lava.iter().min_by_key(|c| c.0).unwrap().0;
    let min_y = lava.iter().min_by_key(|c| c.1).unwrap().1;
    let min_z = lava.iter().min_by_key(|c| c.2).unwrap().2;
    let max_x = lava.iter().max_by_key(|c| c.0).unwrap().0;
    let max_y = lava.iter().max_by_key(|c| c.1).unwrap().1;
    let max_z = lava.iter().max_by_key(|c| c.2).unwrap().2;

    let outside= calc_air(&lava, min_x-1, min_y-1, min_z-1, max_x+1, max_y+1, max_z+1);
    let surface: usize = lava.iter().map(|cube| {
        adjacent_cubes(cube).filter(|adj| outside.contains(adj)).count()
    }).sum();

    println!("[Part2] Total surface area is {}", surface);
}

fn adjacent_cubes(cube: &Cube) -> impl Iterator<Item=Cube> {
    [
        (cube.0 -1, cube.1, cube.2),
        (cube.0 +1, cube.1, cube.2),
        (cube.0, cube.1 -1, cube.2),
        (cube.0, cube.1 +1, cube.2),
        (cube.0, cube.1, cube.2 -1),
        (cube.0, cube.1, cube.2 +1)
    ].into_iter()
}

fn calc_air(lava: &HashSet<Cube>, min_x: i8, min_y: i8, min_z: i8, max_x: i8, max_y: i8, max_z: i8) -> HashSet<Cube> {
    let start = (min_x, min_y, min_z);

    let mut air: HashSet<Cube> = HashSet::new();
    let mut queue: VecDeque<Cube> = VecDeque::from([start]);
    let mut visited: HashSet<Cube> = HashSet::new();

    while let Some(cube) = queue.pop_front() {
        if !visited.contains(&cube) && !lava.contains(&cube) {
            air.insert(cube);

            queue.extend(adjacent_cubes(&cube).filter(|x| x.0 <= max_x && x.1 <= max_y && x.2 <= max_z && x.0 >= min_x && x.1 >= min_y && x.2 >= min_z));
        }

        visited.insert(cube);
    }
    air
}