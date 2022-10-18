mod algo;
mod util;
mod visualize;

use algo::{annealing::Annealing, brute_force::BruteForce};
use rand::Rng;
use std::env;
use visualize::Visualizer;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let area: f64 = args[2]
        .parse()
        .map_err(|e| format!("Please input an area has second argument: {e}"))?;

    let cities = match args[3].trim().to_lowercase().as_str() {
        "in" | "inp" | "input" => Ok(input_cities()?),
        "rand" | "random" => {
            let count = args[4]
                .parse()
                .map_err(|e| format!("Please input a city count as fourth argument: {e}"))?;
            Ok(random_cities(count, area))
        }
        _ => Err(format!("'{}' is no valid input mode!", args[2])),
    }?;

    match args[1].trim().to_lowercase().as_str() {
        "an" | "annealing" => Visualizer::<Annealing>::new(cities, area).run(),
        "bf" | "brute-force" => Visualizer::<BruteForce>::new(cities, area).run(),
        _ => return Err(format!("'{}' is no valid algorithm!", args[1])),
    };
    Ok(())
}

fn random_cities(count: usize, area: f64) -> Vec<(f64, f64)> {
    let mut cities: Vec<(f64, f64)> = Vec::with_capacity(count);
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let x = rng.gen_range(-area / 2.0..area / 2.0);
        let y = rng.gen_range(-area / 2.0..area / 2.0);
        cities.push((x, y));
    }
    cities
}

fn input_cities() -> Result<Vec<(f64, f64)>, String> {
    let mut cities: Vec<(f64, f64)> = Vec::new();

    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.is_empty() {
            break;
        }
        let pos: Vec<&str> = input.trim().split(' ').collect();
        if pos.len() < 2 {
            return Err("Please enter two values.".to_string());
        }
        let x: f64 = pos[0]
            .parse()
            .map_err(|e| format!("Please enter a float: {e}"))?;
        let y: f64 = pos[1]
            .parse()
            .map_err(|e| format!("Please enter a float: {e}"))?;
        cities.push((x, y));
    }
    Ok(cities)
}
