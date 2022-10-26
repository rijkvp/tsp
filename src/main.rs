mod algo;
mod util;
#[cfg(feature = "visualize")]
mod visualize;

use algo::{annealing::Annealing, brute_force::BruteForce};
use rand::Rng;
use std::env;

const AREA_SIZE: f64 = 500.0;

enum Algorithm {
    Annealing,
    BruteForce,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(format!("Please enter at least 2 arguements!"));
    }
    let algo_selection = match args[1].trim().to_lowercase().as_str() {
        "an" | "annealing" => Algorithm::Annealing,
        "bf" | "brute-force" => Algorithm::BruteForce,
         _ => return Err(format!("'{}' is not a valid algorithm: please choose between annealing (an) and brute-force (bf).", args[1])),
    };

    let cities = match args[2].trim().to_lowercase().as_str() {
        "in" | "inp" | "input" => Ok(input_cities()?),
        "rand" | "random" => {
            let count_input = args.get(3).ok_or(format!(
                "Please provide a amount of random cities as the third argument!"
            ))?;
            let count = count_input.parse().map_err(|e| {
                format!("Please input a valid city count number as fourth argument: {e}")
            })?;
            Ok(random_cities(count, AREA_SIZE))
        }
        _ => Err(format!(
            "'{}' is no valid input mode: please chooese between: random [count] or input.",
            args[2]
        )),
    }?;

    #[cfg(feature = "visualize")]
    {
        use crate::visualize::Visualizer;
        if args
            .get(4)
            .map(|s| s.to_lowercase().starts_with("v"))
            .unwrap_or(false)
        {
            match algo_selection {
                Algorithm::Annealing => Visualizer::<Annealing>::new(cities, AREA_SIZE).run(),
                Algorithm::BruteForce => Visualizer::<BruteForce>::new(cities, AREA_SIZE).run(),
            }
            return Ok(());
        }
    }

    let state = match algo_selection {
        Algorithm::Annealing => algo::run::<Annealing>(cities),
        Algorithm::BruteForce => algo::run::<BruteForce>(cities),
    };
    println!("Length: {:.2}, Path: {:?}", state.length, state.path);

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
