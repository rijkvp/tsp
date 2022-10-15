mod annealing;
mod brute_force;
mod util;

use crate::annealing::Params;
use rand::Rng;
use std::{env, time::Instant};

const MIN_CITIES: usize = 2; //minimum number of cities
const MAX_CITIES: usize = 50; //maximum number of cities
const AREA_SIZE: f64 = 1000.0;

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

    match args[3].trim().to_lowercase().as_str() {
        "an" | "anneal" | "annealing" => {
            let (len, path) = annealing::run_annealing(cities, Params::default());
            println!("Length: {len:.2}, Path: {path:?}");
        }
        "anc" | "anneal_custom" | "annealing_custom" => { //annealing with user-inputted start parameters
            let (len, path) = annealing::run_annealing(cities, annealing::user_input_params());
            println!("Length: {len:.2}, Path: {path:?}");
        }
        "bf" | "brute-force" => {
            let (len, path) = brute_force::run_brute_force(cities);
            println!("Length: {len:.2}, Path: {path:?}");
        }
        "cmp" | "compare" => {
            compare(cities, true);
        }
        "cmpc" | "compare_custom" => {
            compare(cities, false);
        }
        "test" => {
            fn rand_param() -> Params {
                let mut rng = rand::thread_rng();
                Params {
                    start_temp: rng.gen_range(0.0..10.0),
                    temp_mult: rng.gen_range(0.9..0.999),
                    max_iter: rng.gen_range(1000..100000),
                    max_nodecrease: rng.gen_range(10..100),
                }
            }
            let (bf_len, bf_path) = brute_force::run_brute_force(cities.clone());
            let mut best_len = f64::MAX;
            let mut best_param;
            let mut best_path;
            loop {
                let param = rand_param();
                let (len, path) = annealing::run_annealing(cities.clone(), param);
                if len <= best_len {
                    best_len = len;
                    best_param = param;
                    best_path = path;
                    let score = (bf_len / len) * 100.0;
                    println!(
                        "{score:.1}%\tlength={:.2}\tparam={:?}\tpath=\t{:?}\top={:.2}\tbest={:?}\t",
                        best_len, best_param, best_path, bf_len, bf_path
                    );
                }
            }
        }
        _ => return Err(format!("'{}' is no valid input mode!", args[3])),
    };
    Ok(())
}

fn compare(cities: Vec<(f64, f64)>, use_default: bool) {

    //first do the simulated annealing
    let an_duration;
    let (an_len, an_path);
    if use_default {
        let start_time_an = Instant::now();
        (an_len, an_path) = annealing::run_annealing(cities.clone(), Params::default());
        an_duration = start_time_an.elapsed();
    } else {
        let user_params = annealing::user_input_params();
        let start_time_an = Instant::now();
        (an_len, an_path) = annealing::run_annealing(cities.clone(), user_params);
        an_duration = start_time_an.elapsed();
    }

    //now do the brute force
    let start_time_bf = Instant::now();
    let (bf_len, bf_path) = brute_force::run_brute_force(cities.clone());
    let bf_duration = start_time_bf.elapsed();

    println!("==Algorithm Comparison==");
    println!("Brute-Force:\t{bf_len:.2}\t{bf_path:?}\t{bf_duration:?}");
    println!("Annealing:  \t{an_len:.2}\t{an_path:?}\t{an_duration:?}");
    println!("Annealing length: {:.2}%", an_len / bf_len * 100.0);
    println!(
        "Annealing time: {:.2}%",
        an_duration.as_secs_f64() / bf_duration.as_secs_f64() * 100.0
    );
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
