use std::ops::{Add, AddAssign, Sub, SubAssign};

use aoc22::read_aoc_file;

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources
}

fn main() {
    let blueprints: Vec<Blueprint> = read_aoc_file(19).map(|line| {
        match line.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
            ["Blueprint", id, 
                "Each", "ore", "robot", "costs", ore_robot_ore_cost, "ore.", 
                "Each", "clay", "robot", "costs", clay_robot_ore_cost, "ore.", 
                "Each", "obsidian", "robot", "costs", obsidian_robot_ore_cost, "ore", "and", obsidian_robot_clay_cost, "clay.", 
                "Each", "geode", "robot", "costs", geode_robot_ore_cost, "ore", "and", geode_robot_obsidian_cost, "obsidian."
            ] => {
                let id: u8 = id.strip_suffix(":").unwrap().parse().unwrap();
                Blueprint {
                    id,
                    ore_robot: Resources { ore: ore_robot_ore_cost.parse().unwrap(), clay: 0, obsidian: 0, geodes: 0 },
                    clay_robot: Resources { ore: clay_robot_ore_cost.parse().unwrap(), clay: 0, obsidian: 0, geodes: 0 },
                    obsidian_robot: Resources { ore: obsidian_robot_ore_cost.parse().unwrap(), clay: obsidian_robot_clay_cost.parse().unwrap(), obsidian: 0, geodes: 0 },
                    geode_robot: Resources { ore: geode_robot_ore_cost.parse().unwrap(), clay: 0, obsidian: geode_robot_obsidian_cost.parse().unwrap(), geodes: 0 },
                }
            },
            _ => panic!("Invalid blueprint: \"{}\"", line)
        }
    }).collect();

    println!("{:?}", blueprints);

    let scores: Vec<_> = blueprints.iter().skip(1).map(|bp| (bp.id, play_game(bp, 24))).collect();
    println!("{:?}", scores);
}

#[derive(Debug,Clone,Copy,Default)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geodes: self.geodes + rhs.geodes
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geodes += rhs.geodes;
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geodes: self.geodes - rhs.geodes
        }
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geodes -= rhs.geodes;
    }
}

macro_rules! impl_build_roboter {
    ( $func:ident, $roboter:ident, $res_a:ident $(, $res_b:ident)? ) => {
        fn $func(cost: &Resources, res: &mut Resources, robos: &mut Resources) -> bool {
            if res.$res_a >= cost.$res_a $(&& res.$res_b >= cost.$res_b)? {
                robos.$roboter = 1;
                res.$res_a -= cost.$res_a;
                $(res.$res_b -= cost.$res_b;)?
                return true;
            }
        
            false
        }
    };
}
impl_build_roboter!(build_ore_roboter, ore, ore);
impl_build_roboter!(build_clay_roboter, clay, ore);
impl_build_roboter!(build_obsidian_roboter, obsidian, ore, clay);
impl_build_roboter!(build_geode_roboter, geodes, ore, obsidian);

macro_rules! calc_remaining_time {
    ( $cost:expr, $res:expr, $robos:expr, res=[$res_a:ident, $res_b:ident] $(, build=[$build_robo_costs:expr, $build_robo:ident])? ) => {
        {
            let robots_a = $robos.$res_a $(+ if stringify!($build_robo) == stringify!($res_a) {1} else {0})?;
            let robots_b = $robos.$res_b $(+ if stringify!($build_robo) == stringify!($res_b) {1} else {0})?;

            let time_a: f32 = if robots_a != 0 { ($cost.$res_a as f32 - $res.$res_a as f32 $(+ $build_robo_costs.$res_a as f32)? ) / robots_a as f32 } else { f32::MAX };
            let time_b: f32 = if robots_b != 0 { ($cost.$res_b as f32 - $res.$res_b as f32 $(+ $build_robo_costs.$res_b as f32)? ) / robots_b as f32 } else { f32::MAX };
            let time: u8 = f32::max(time_a, time_b).clamp(0.0, 255.0) as u8;
            (time_a, time_b, time)
        }

    };
}


fn play_game(bp: &Blueprint, minutes: u8) -> u8 {
    let mut res = Resources::default();
    let mut robo = Resources::default();
    robo.ore = 1;

    let mut queue = Resources::default();
    for minute in 1..=minutes {
        // Try to build an geode roboter
        if !build_geode_roboter(&bp.geode_robot, &mut res, &mut queue) {
            // Can't build a geode roboter. Checking which resource is bottleneck
            let (ore_ratio, obsidian_ratio, geode_time): (f32, f32, u8) = calc_remaining_time!(bp.geode_robot, res, robo, res=[ore, obsidian]);
            
            if ore_ratio > obsidian_ratio {
                // Try to build an ore roboter
                let (_, _, new_time) = calc_remaining_time!(bp.geode_robot, res, robo, res=[ore, obsidian], build=[bp.ore_robot, ore]);

                println!("M{} [G] Ore roboter: {} vs {}", minute, new_time, geode_time);
                if new_time <= geode_time {
                    build_ore_roboter(&bp.ore_robot, &mut res, &mut queue);
                }
            } else {
                // Try to build an obsidian roboter
                let (_, _, new_time) = calc_remaining_time!(bp.geode_robot, res, robo, res=[ore, obsidian], build=[bp.obsidian_robot, obsidian]);

                println!("M{} [G] Obsidian roboter: {} vs {}", minute, new_time, geode_time);
                if new_time <= geode_time  {
                    if !build_obsidian_roboter(&bp.obsidian_robot, &mut res, &mut queue) {
                        // Can't build an obsidian roboter. Checking which resource is bottleneck
                        let (ore_ratio, clay_ratio, obsidian_time): (f32, f32, u8) = calc_remaining_time!(bp.obsidian_robot, res, robo, res=[ore, clay]);
    
                        if ore_ratio > clay_ratio {
                            // Try to build an ore roboter
                            let (_, _, new_time_geode) = calc_remaining_time!(bp.geode_robot, res, robo, res=[ore, obsidian], build=[bp.ore_robot, ore]);
                            let (_, _, new_time_obsidian) = calc_remaining_time!(bp.obsidian_robot, res, robo, res=[ore, clay], build=[bp.ore_robot, ore]);

                            println!("M{} [O] Ore roboter: {} vs {} and {} vs {}", minute, new_time_geode, geode_time, new_time_obsidian, obsidian_time);
                            if new_time_geode <= geode_time && new_time_obsidian <= obsidian_time  {
                                build_ore_roboter(&bp.ore_robot, &mut res, &mut queue);
                            }
                        } else {
                            // Try to build an clay roboter
                            let (_, _, new_time_geode) = calc_remaining_time!(bp.geode_robot, res, robo, res=[ore, obsidian], build=[bp.clay_robot, clay]);
                            let (_, _, new_time_obsidian) = calc_remaining_time!(bp.obsidian_robot, res, robo, res=[ore, clay], build=[bp.clay_robot, clay]);

                            println!("M{} [O] Clay roboter: {} vs {} and {} vs {}", minute, new_time_geode, geode_time, new_time_obsidian, obsidian_time);
                            if new_time_geode <= geode_time && new_time_obsidian <= obsidian_time {
                                if !build_clay_roboter(&bp.clay_robot, &mut res, &mut queue) {
                                    // Can't build an clay roboter
                                    let (_, _, clay_time): (f32, f32, u8) = calc_remaining_time!(bp.clay_robot, res, robo, res=[ore, ore]);

                                    // Try to build an ore roboter
                                    let (_, _, new_time_geode) = calc_remaining_time!(bp.geode_robot, res, robo, res=[ore, obsidian], build=[bp.ore_robot, ore]);
                                    let (_, _, new_time_obsidian) = calc_remaining_time!(bp.obsidian_robot, res, robo, res=[ore, clay], build=[bp.ore_robot, ore]);
                                    let (_, _, new_time_clay) = calc_remaining_time!(bp.clay_robot, res, robo, res=[ore, ore], build=[bp.ore_robot, ore]);
        
                                    println!("M{} [C] Ore roboter: {} vs {} and {} vs {} and {} vs {}", minute, new_time_geode, geode_time, new_time_obsidian, obsidian_time, new_time_clay, clay_time);
                                    if new_time_geode <= geode_time && new_time_obsidian <= obsidian_time && new_time_clay <= clay_time {
                                        build_ore_roboter(&bp.ore_robot, &mut res, &mut queue);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Robots generate resources
        res += robo;

        println!("M{} Resor: {:?}", minute, res);
        println!("M{} Robos: {:?}", minute, robo);
        println!("M{} Queue: {:?}", minute, queue);

        // New robot is ready; Add it
        robo += queue;
        // Clear production queue
        queue -= queue;
    }

    res.geodes
}