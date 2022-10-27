use super::{TspAlgorithm, TspState};

pub struct BruteForce {
    cities: Vec<(f64, f64)>,
    perm: Vec<usize>,
    p_count: u128,
    max: u128,
    length: f64,
    path: Vec<usize>,
}

impl TspAlgorithm for BruteForce {
    fn init(cities: Vec<(f64, f64)>) -> BruteForce {
        let path = (0..cities.len()).collect();
        let max = factorial(cities.len() as u128);
        Self {
            length: calculate_length(&cities, &path),
            perm: path.clone(),
            path,
            cities,
            p_count: 0,
            max,
        }
    }

    fn state(&self) -> TspState {
        let p = self.p_count * 100 / self.max;
        TspState {
            length: self.length,
            path: self.path.clone(),
            sample: self.perm.clone(),
            status: format!("P: {} {}%", self.p_count, p),
        }
    }

    fn step(&mut self) -> bool {
        if !next_permutation(&mut self.perm) {
            return true;
        }
        self.p_count += 1;
        let new_length = calculate_length(&self.cities, &self.perm);
        if new_length < self.length {
            self.length = new_length;
            self.path = self.perm.clone();
        }
        false
    }
}

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

// Calculates the length of a path
fn calculate_length(cities: &Vec<(f64, f64)>, path: &Vec<usize>) -> f64 {
    let mut length = 0.0;
    for i in 0..path.len() {
        let x = cities[path[i]];
        let y = cities[path[(i + 1) % path.len()]];
        length += dist(&x, &y);
    }
    length
}

// Calculates the distance between two points
fn dist(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx * dx + dy * dy).sqrt()
}

// Lexicographic permutation algorithm: https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
fn next_permutation(perm: &mut Vec<usize>) -> bool {
    let mut k = None;
    let len = perm.len();
    for i in 0..len - 1 {
        if perm[i] < perm[i + 1] {
            k = Some(i);
        }
    }
    if let Some(k) = k {
        let mut l = k;
        for i in k..len {
            if perm[i] > perm[k] {
                l = i;
            }
        }
        // Swap k and l
        perm.swap(k, l);
        // Reverse k+1 till end
        perm[k + 1..len].reverse();
        return true;
    }
    false
}
