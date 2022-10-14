mod brute_force;
mod util;
mod annealing;

use std::env;

const MIN_CITIES: usize = 2; //minimum number of cities
const MAX_CITIES: usize = 50; //maximum number of cities

fn main() {
    println!("Hey there, welcome to the TSP program by Rijk van Putten and Aron Hardeman");

    if let Err(e) = run() {
        eprintln!("Error: {e}");
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(format!("You entered {} arugments. Please enter exactly 3!", args.len()))
    }
    let cities = match args[1].trim().to_lowercase().as_str() {
        "in" | "inp" | "input" => Ok(input_cities()),
        "rand" | "random" => Ok(random_cities()),
        _ => Err(format!("'{}' is no valid input mode!", args[1]))
    }?;
    match args[2].trim().to_lowercase().as_str() {
        "an" | "anneal" | "annealing" => Ok(annealing::run_annealing(cities)),
        "bf" | "brute-force" => Ok(brute_force::run_brute_force(cities)),
        _ => Err(format!("'{}' is no valid input mode!", args[1]))
    }?;
    Ok(())
}

fn random_cities() -> Vec<(f64, f64)> {
    todo!()
}

fn input_cities() -> Vec<(f64, f64)> {
    println!(
        "How many cities would you like to have? {} <= n <= {}",
        MIN_CITIES, MAX_CITIES
    );

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Woops, could not read your input.");

    let n: usize = input
        .trim()
        .parse()
        .expect("You should type a number, byebye");

    if n > MAX_CITIES || n < MIN_CITIES {
        eprintln!("Your input is not within the specified bounds.");
        std::process::exit(1);
    }
    let mut cities: Vec<(f64, f64)> = Vec::new();

    for i in 0..n {
        println!(
            "Enter x and y locations of city #{} (separated by a space) and press enter:",
            i
        );
        std::io::stdin().read_line(&mut input).expect("Input failed");
        let current_position: Vec<f64> = input
            .trim()
            .split(' ')
            .map(|x| x.parse().expect("input a number please"))
            .collect();
        cities.push((current_position[0], current_position[1]));
    }
    debug_assert!(cities.len() == n);
    return cities;
}