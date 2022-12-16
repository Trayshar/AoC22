use std::{collections::HashMap, cell::RefCell, rc::Rc, fmt::Display, io::{stdout, Write}};

use aoc22::read_aoc_file;

#[derive(Debug, Clone)]
struct Valve{
    tag: String,
    rate: u32,
    tunnels: Box<[String]>
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("Valve {} ({}) to {:?}", self.tag, self.rate, self.tunnels))
    }
}

#[derive(Debug)]
struct Game{
    time: u32,
    pressure: u32,
    current: String,
    opened: Vec<String>
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("Game (time {}, cur \"{}\", pressure {}, opened {:?})", self.time, self.current, self.pressure, self.opened))
    }
}

fn main() {
    let valves: Vec<Valve> = read_aoc_file(16).map(|line| {
        match line.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
            ["Valve", tag, "has", "flow", rate, _tunnels, _lead, "to", _valves, tunnels @ ..] => {
                let rate: u32 = rate.strip_prefix("rate=").unwrap().strip_suffix(";").unwrap().parse().unwrap();
                Valve{
                    tag: tag.to_string(),
                    rate,
                    tunnels: tunnels.iter().map(|t| t.replace(",", "")).collect()
                }
            },
            _ => panic!("Unknown valve data: \"{}\"", line)
        }
    }).collect();
    let start = valves[0].tag.clone();
    let mut valves: HashMap<String, Valve> = valves.into_iter().map(|valve| (valve.tag.clone(), valve)).collect();

    let temp = valves.clone();
    for (_, valve) in valves.iter_mut() {
        valve.tunnels.sort_unstable_by_key(|t| temp.get(t).unwrap().rate);
        valve.tunnels.reverse();
    }

    println!("Valves: {:?}", valves);

    let highest_pressure = Rc::new(RefCell::new(1000u32));
    let winner = play_game(&valves, highest_pressure, Game { 
        time: 30, 
        pressure: 0, 
        current: start,
        opened: Vec::new()
    }).unwrap();
    println!("\nThe maximum pressure is {}", winner.pressure);
}

fn calc_max_pressure(valves: &HashMap<String, Valve>, game: &Game) -> u32 {
    valves.iter().filter_map(|(tag, valve)| {
        if game.opened.contains(tag) { return None;}
        Some(valve.rate * (game.time - 1))
    }).sum::<u32>() + game.pressure
}

fn play_game(valves: &HashMap<String, Valve>, highest_pressure: Rc<RefCell<u32>>, game: Game) -> Option<Game> {
    if game.pressure > *RefCell::borrow(&highest_pressure) {
        *RefCell::borrow_mut(&highest_pressure) = game.pressure;
    }

    let h = *RefCell::borrow(&highest_pressure);
    if game.time == 0 { return Some(game) };
    if h > calc_max_pressure(valves, &game) { return None; }

    
    print!("\rPlay {:80}, highest {}", game, h);

    let valve = valves.get(&game.current).expect("Unknown valve tag!");

    let games = valve.tunnels.iter().map(|next| {
        play_game(valves, highest_pressure.clone(), Game { 
            time: game.time - 1, 
            pressure: game.pressure, 
            current: next.clone(),
            opened: game.opened.clone()
        })
    });

    if !game.opened.contains(&game.current) && valve.rate > 0 {
        let mut opened = game.opened.clone();
        opened.push(game.current.clone());
        std::iter::once(play_game(valves, highest_pressure.clone(), Game { 
            time: game.time - 1, 
            pressure: game.pressure + valve.rate * (game.time - 1), 
            current: game.current.clone(),
            opened
        })).chain(games).flatten().max_by_key(|game| game.pressure)
    }else {
        games.flatten().max_by_key(|game| game.pressure)
    }
}