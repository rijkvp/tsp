use std::f64::consts;

use crate::util;
use rand::Rng;

const START_TEMP: f64 = 200.0; // The starting temperature
const TEMP_MULT: f64 = 0.999; // The multiplier of the temperature after each step

pub fn run_annealing(cities: Vec<(f64, f64)>) -> (f64, Vec<usize>) {
    eprintln!("Annealing..");
    let mut rng = rand::thread_rng();
    let n = cities.len();

    // Matrix of the squared distances between cities; size n*n
    let mut distance: Vec<Vec<f64>> = vec![vec![-1.0; n]; n];
    for i in 0..n {
        for j in i..n {
            distance[i][j] = util::dist(&cities[i], &cities[j]);
            distance[j][i] = distance[i][j];
        }
    }

    // Initialize the path
    // TODO: Should we also randomize the path before starting?
    let mut path: Vec<usize> = vec![usize::MAX; n];
    for i in 0..n {
        path[i] = i as usize;
    }

    let mut start_dist = 0.0;
    for i in 1..n {
        let x = path[i];
        start_dist += distance[x - 1][x];
    }

    let mut temperature = START_TEMP;
    let mut i = 0;
    let mut curr_path = path;
    let mut curr_dist = start_dist;
    loop {
        // Quit after a certain amount of iterations
        if i > 10000 {
            break;
        }
        // Apply a random action to the path
        let new_path = match rng.gen_range(0..2) {
            0 => swap_cities(&curr_path),
            1 => invert_section(&curr_path),
            2 => shift(&curr_path),
            _ => panic!(),
        };
        // Calculate the distance of the new path
        let mut new_dist = 0.0;
        for i in 1..n {
            let x = new_path[i];
            let y = new_path[i - 1];
            if x > 0 {
                new_dist += distance[x][y];
            }
        }

        // Difference in energy level is the difference in distance
        let delta_e = curr_dist - new_dist;
        let accept = {
            if delta_e >= 0.0 {
                // Always accept a smaller paths with a lower energy level
                true
            } else {
                // Only accept paths arbitrary according the probability formula
                let probability = consts::E.powf(delta_e / temperature);
                rng.gen_bool(probability)
            }
        };
        if accept {
            curr_path = new_path;
            curr_dist = new_dist;
        }
        temperature *= TEMP_MULT; // Decrease temerature
        i += 1;
    }
    eprintln!("Annealed from {:.2} to {:.2}.", start_dist, curr_dist);
    return (curr_dist, curr_path);
}

// Generate two distinct random numbers
fn distinct_indicies(max: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..max);
    let mut y = x;
    while y == x {
        y = rng.gen_range(0..max);
    }
    (x, y)
}

// Swap the elements of two random indicies
fn swap_cities(path: &[usize]) -> Vec<usize> {
    let mut res = Vec::from(path);
    let (x, y) = distinct_indicies(path.len());
    util::swap(&mut res, x, y);
    res
}

// Invert the order of a random section
fn invert_section(path: &[usize]) -> Vec<usize> {
    let (x, y) = distinct_indicies(path.len());
    let (s, e) = if x < y { (x, y) } else { (y, x) };
    let mut res = Vec::with_capacity(path.len());
    for i in 0..path.len() {
        if i >= s && i <= e {
            res.push(path[e - (i - s)]);
        } else {
            res.push(path[i]);
        }
    }
    res
}

// Peform a circular shift left or right
fn shift(path: &[usize]) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        util::shift_right(path)
    } else {
        util::shift_left(path)
    }
}
