use super::{TspAlgorithm, TspState};
use rand::Rng;
use std::f64::consts;

#[derive(Debug, Copy, Clone)]
pub struct Params {
    // Starting temperature
    start_temp: f64,
    // Multiplier of the temperature: usually between 0.9 and 0.999
    temp_mult: f64,
    // Maximum steps
    max_steps: usize,
    // How many candidates to pick each step
    candidates: usize,
    // Maximum steps without energy decrease before stopping
    max_nodecrease: usize,
}

// The default parameters of the annealing algorithm
impl Default for Params {
    fn default() -> Self {
        Self {
            start_temp: 30.0,
            temp_mult: 0.95,
            candidates: 200,
            max_steps: 200,
            max_nodecrease: 50,
        }
    }
}

pub struct Annealing {
    param: Params,
    distance: Vec<Vec<f64>>,
    temperature: f64,
    step: usize,
    candidate: usize,
    last_decrease: usize,
    path: Vec<usize>,
    sample: Vec<usize>,
    length: f64,
}

impl Annealing {
    pub fn new(cities: Vec<(f64, f64)>, param: Params) -> Self {
        let mut rng = rand::thread_rng();

        // Matrix of the squared distances between cities; size n*n
        let mut distance: Vec<Vec<f64>> = vec![vec![-1.0; cities.len()]; cities.len()];
        for i in 0..cities.len() {
            for j in i..cities.len() {
                distance[i][j] = dist(&cities[i], &cities[j]);
                distance[j][i] = distance[i][j];
            }
        }

        // Initialize a random path
        let mut path: Vec<usize> = Vec::with_capacity(cities.len());
        for i in 0..cities.len() {
            let rand_index = rng.gen_range(0..path.len() + 1);
            path.insert(rand_index, i)
        }

        let mut length = 0.0;
        for i in 0..cities.len() {
            length += distance[path[i]][path[(i + 1) % cities.len()]];
        }

        Self {
            param,
            distance,
            temperature: param.start_temp,
            step: 0,
            candidate: 0,
            last_decrease: 0,
            sample: path.clone(),
            path,
            length,
        }
    }
}

impl TspAlgorithm for Annealing {
    fn init(cities: Vec<(f64, f64)>) -> Self {
        Self::new(cities, Params::default())
    }

    fn state(&self) -> TspState {
        TspState {
            length: self.length,
            path: self.path.clone(),
            sample: self.sample.clone(),
            status: format!(
                "S={:<3} C={:<3}  T={:.3}",
                self.step, self.candidate, self.temperature
            ),
        }
    }

    fn step(&mut self) -> bool {
        let mut rng = rand::thread_rng();

        if self.candidate < self.param.candidates {
            // Apply a random action to the path
            self.sample = match rng.gen_range(0..=2) {
                0 => swap_cities(&self.path),
                1 => invert_section(&self.path),
                2 => shift(&self.path),
                _ => panic!(),
            };

            // Calculate the distance of the new path
            let mut new_length = 0.0;
            for i in 0..self.sample.len() {
                let x = self.sample[i];
                let y = self.sample[(i + 1) % self.sample.len()];
                new_length += self.distance[x][y];
            }

            // Difference in energy level is the difference in distance
            let delta_e = self.length - new_length;
            let accept = {
                if delta_e >= 0.0 {
                    // Always accept a smaller paths with a lower energy level
                    true
                } else {
                    // Only accept paths arbitrary according the probability formula
                    let probability = consts::E.powf(delta_e / self.temperature);
                    rng.gen_bool(probability)
                }
            };
            if accept {
                self.path = self.sample.clone();
                self.length = new_length;
                self.last_decrease = self.step;
            }
            self.candidate += 1;
        } else {
            self.candidate = 0;
            // Check for stop conditions
            if self.step > self.param.max_steps
                || self.step - self.last_decrease > self.param.max_nodecrease
            {
                return true;
            } else {
                self.temperature *= self.param.temp_mult; // Decrease temerature
                self.step += 1;
            }
        }
        false
    }
}

// Calculates the distance between two points
fn dist(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx * dx + dy * dy).sqrt()
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
    res.swap(x, y);
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
        shift_right(path)
    } else {
        shift_left(path)
    }
}

// Returns a new vector with all elements circularly shifted to the right
fn shift_right(arr: &[usize]) -> Vec<usize> {
    let mut res = Vec::with_capacity(arr.len());
    res.push(arr[arr.len() - 1]);
    for i in 0..arr.len() - 1 {
        res.push(arr[i]);
    }
    res
}

// Returns a new vector with all elements circularly shifted to the left
fn shift_left(arr: &[usize]) -> Vec<usize> {
    let mut res = Vec::with_capacity(arr.len());
    for i in 1..arr.len() {
        res.push(arr[i]);
    }
    res.push(arr[0]);
    res
}
