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

    let scores: Vec<_> = blueprints.iter().take(1).map(|bp| (bp.id, play_game(bp, 24))).collect();
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

macro_rules! calc_ratio {
    ( $cost:expr, $res:expr, $robos:expr, $res_a:ident, $res_b:ident ) => {
        (
            if $robos.$res_a != 0 { ($cost.$res_a as f32 - $res.$res_a as f32) / $robos.$res_a as f32 } else { f32::MAX }, 
            if $robos.$res_b != 0 { ($cost.$res_b as f32 - $res.$res_b as f32) / $robos.$res_b as f32 } else { f32::MAX }
        )
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
            let (ore_ratio, obsidian_ratio): (f32, f32) = calc_ratio!(bp.geode_robot, res, robo, ore, obsidian);

            if ore_ratio > obsidian_ratio && (robo.ore + 1) * (minutes - minute) + res.ore >= bp.geode_robot.ore {
                // Try to build an ore roboter
                build_ore_roboter(&bp.ore_robot, &mut res, &mut queue);
            } else if (robo.obsidian + 1) * (minutes - minute) + res.obsidian >= bp.geode_robot.obsidian {
                // Try to build an obsidian roboter
                if !build_obsidian_roboter(&bp.obsidian_robot, &mut res, &mut queue) {
                    // Can't build an obsidian roboter. Checking which resource is bottleneck
                    let (ore_ratio, clay_ratio): (f32, f32) = calc_ratio!(bp.obsidian_robot, res, robo, ore, clay);

                    if ore_ratio > clay_ratio && (robo.ore + 1) * (minutes - minute) + res.ore >= bp.obsidian_robot.ore && (robo.ore + 1) * (minutes - minute) + res.ore >= bp.geode_robot.ore {
                        // Try to build an ore roboter
                        build_ore_roboter(&bp.ore_robot, &mut res, &mut queue);
                    } else if (robo.clay + 1) * (minutes - minute) + res.clay >= bp.obsidian_robot.clay {
                        // Try to build an clay roboter
                        build_clay_roboter(&bp.clay_robot, &mut res, &mut queue);
                    }
                }
            }
        }

        // Robots generate resources
        res += robo;

        println!("M{}", minute);
        println!("Resor: {:?}", res);
        println!("Robos: {:?}", robo);
        println!("Queue: {:?}", queue);

        // New robot is ready; Add it
        robo += queue;
        // Clear production queue
        queue -= queue;
    }

    res.geodes
}