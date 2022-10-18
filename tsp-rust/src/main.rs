mod algo;
mod util;
mod visualize;

use algo::{annealing::Annealing, brute_force::BruteForce};
use rand::Rng;
use std::env;
use visualize::Visualizer;

const MIN_CITIES: usize = 2; //minimum number of cities
pub const AREA_SIZE: f64 = 500.0;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err(format!(
            "You entered {} arguments. Please provide exactly 3!",
            args.len() - 1
        ));
    }
    let count: usize = args[3]
        .parse()
        .map_err(|e| format!("Please input a number has third argument: {e}"))?;
    if count < MIN_CITIES {
        return Err(format!(
            "Please enter a city count of at least {}",
            MIN_CITIES
        ));
    }
    let cities = match args[2].trim().to_lowercase().as_str() {
        "in" | "inp" | "input" => Ok(input_cities(count)?),
        "rand" | "random" => Ok(random_cities(count)),
        _ => Err(format!("'{}' is no valid input mode!", args[2])),
    }?;

    match args[1].trim().to_lowercase().as_str() {
        "an" | "annealing" => Visualizer::<Annealing>::new(cities).run(),
        "bf" | "brute-force" => Visualizer::<BruteForce>::new(cities).run(),
        _ => return Err(format!("'{}' is no valid algorithm!", args[1])),
    };
    Ok(())
}

fn random_cities(count: usize) -> Vec<(f64, f64)> {
    let mut cities: Vec<(f64, f64)> = Vec::with_capacity(count);
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let x = rng.gen_range(-AREA_SIZE..AREA_SIZE);
        let y = rng.gen_range(-AREA_SIZE..AREA_SIZE);
        cities.push((x, y));
    }
    cities
}

fn input_cities(count: usize) -> Result<Vec<(f64, f64)>, String> {
    let mut cities: Vec<(f64, f64)> = Vec::with_capacity(count);

    let mut input = String::new();
    for i in 0..count {
        println!("Enter x and y locations of city #{}", i);
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
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
    debug_assert!(cities.len() == count);
    Ok(cities)
}
