mod annealing;
mod brute_force;
mod util;

use rand::Rng;
use std::{env, time::Instant};

const MIN_CITIES: usize = 2; //minimum number of cities
const MAX_CITIES: usize = 50; //maximum number of cities

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err(format!(
            "You entered {} arguments. Please provide exactly 4!",
            args.len()
        ));
    }
    let count: usize = args[1]
        .parse()
        .map_err(|e| format!("Please input a number has first argument: {e}"))?;
    if count > MAX_CITIES || count < MIN_CITIES {
        return Err(format!(
            "Please enter a number in range {} - {}",
            MIN_CITIES, MAX_CITIES
        ));
    }
    let cities = match args[2].trim().to_lowercase().as_str() {
        "in" | "inp" | "input" => Ok(input_cities(count)?),
        "rand" | "random" => Ok(random_cities(count)),
        _ => Err(format!("'{}' is no valid input mode!", args[2])),
    }?;

    let (len, path) = match args[3].trim().to_lowercase().as_str() {
        "an" | "anneal" | "annealing" => Ok(annealing::run_annealing(cities)),
        "bf" | "brute-force" => Ok(brute_force::run_brute_force(cities)),
        "cmp" | "compare" => {
            let start_time = Instant::now();
            let (bf_len, bf_path) = brute_force::run_brute_force(cities.clone());
            let bf_duration = start_time.elapsed();
            let (an_len, an_path) = annealing::run_annealing(cities);
            let an_duration = start_time.elapsed() - bf_duration;
            println!("==Algorithm Comparison==");
            println!("Brute-Force:\t{bf_len:.2}\t{bf_path:?}\t{bf_duration:?}");
            println!("Annealing:  \t{an_len:.2}\t{an_path:?}\t{an_duration:?}");
            println!("Annealing length: {:.2}%", an_len / bf_len * 100.0);
            println!(
                "Annealing time: {:.2}%",
                an_duration.as_secs_f64() / bf_duration.as_secs_f64() * 100.0
            );
            Ok((bf_len, bf_path))
        }
        _ => Err(format!("'{}' is no valid input mode!", args[3])),
    }?;
    println!("Length: {len:.2}, Path: {path:?}");
    Ok(())
}

fn random_cities(count: usize) -> Vec<(f64, f64)> {
    let mut cities: Vec<(f64, f64)> = Vec::with_capacity(count);
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let x = rng.gen_range(-1000.0..1000.0);
        let y = rng.gen_range(-1000.0..1000.0);
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
