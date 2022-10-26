mod algo;
mod util;
#[cfg(feature = "visualize")]
mod visualize;

use algo::{annealing::Annealing, brute_force::BruteForce};
use rand::Rng;
use std::env;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

enum AlgoSelection {
    Annealing,
    BruteForce,
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return Err(format!("Please enter at least 3 arguements!"));
    }
    let algo_selection = match args[1].trim().to_lowercase().as_str() {
        "an" | "annealing" => AlgoSelection::Annealing,
        "bf" | "brute-force" => AlgoSelection::BruteForce,
         _ => return Err(format!("'{}' is not a valid algorithm: please choose between annealing (an) and brute-force (bf).", args[1])),
    };

    let area: f64 = args[2]
        .parse()
        .map_err(|e| format!("Please input an area has second argument: {e}"))?;

    let cities = match args[3].trim().to_lowercase().as_str() {
        "in" | "inp" | "input" => Ok(input_cities()?),
        "rand" | "random" => {
            let count_input = args.get(4).ok_or(format!(
                "Please enter a random city count as fourth argument!"
            ))?;
            let count = count_input.parse().map_err(|e| {
                format!("Please input a valid city count number as fourth argument: {e}")
            })?;
            Ok(random_cities(count, area))
        }
        _ => Err(format!(
            "'{}' is no valid input mode: please chooese between: random [count] or input.",
            args[3]
        )),
    }?;

    #[cfg(feature = "visualize")]
    {
        use crate::visualize::Visualizer;
        if args
            .get(5)
            .map(|s| s.to_lowercase().starts_with("vis"))
            .unwrap_or(false)
        {
            match algo_selection {
                AlgoSelection::Annealing => Visualizer::<Annealing>::new(cities, area).run(),
                AlgoSelection::BruteForce => Visualizer::<BruteForce>::new(cities, area).run(),
            }
            return Ok(());
        }
    }

    let state = match algo_selection {
        AlgoSelection::Annealing => algo::run::<Annealing>(cities),
        AlgoSelection::BruteForce => algo::run::<BruteForce>(cities),
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
