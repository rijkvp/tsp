use super::TspAlgorithm;
use crate::util;
use rand::Rng;
use std::f64::consts;

#[derive(Debug, Copy, Clone)]
pub struct Params {
    /// Multiplier of the temperature: usually between 0.9 and 0.999
    pub temp_mult: f64,
    /// Maximum steps
    pub max_steps: usize,
    /// How many candidates to pick each step
    pub candidates: usize,
    // Maximum steps without energy decrease before stopping
    pub max_nodecrease: usize,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            temp_mult: 0.95,
            candidates: 500,
            max_steps: 500,
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
    length: f64,
}

impl Annealing {
    pub fn new(cities: Vec<(f64, f64)>, param: Params) -> Self {
        let mut rng = rand::thread_rng();

        // Matrix of the squared distances between cities; size n*n
        let mut distance: Vec<Vec<f64>> = vec![vec![-1.0; cities.len()]; cities.len()];
        for i in 0..cities.len() {
            for j in i..cities.len() {
                distance[i][j] = util::dist(&cities[i], &cities[j]);
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
            temperature: 1.0,
            step: 0,
            candidate: 0,
            last_decrease: 0,
            path,
            length,
        }
    }
}

impl TspAlgorithm for Annealing {
    fn init(cities: Vec<(f64, f64)>) -> Self {
        Self::new(cities, Params::default())
    }

    fn state(&self) -> (f64, &Vec<usize>, String) {
        (
            self.length,
            &self.path,
            format!(
                "S: {} C: {} T: {:.6}",
                self.step, self.candidate, self.temperature
            ),
        )
    }

    fn step(&mut self) -> bool {
        let mut rng = rand::thread_rng();

        if self.candidate < self.param.candidates {
            // Apply a random action to the path
            let new_path = match rng.gen_range(0..=2) {
                0 => swap_cities(&self.path),
                1 => invert_section(&self.path),
                2 => shift(&self.path),
                _ => panic!(),
            };

            // Calculate the distance of the new path
            let mut new_length = 0.0;
            for i in 0..new_path.len() {
                let x = new_path[i];
                let y = new_path[(i + 1) % new_path.len()];
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
                self.path = new_path;
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
