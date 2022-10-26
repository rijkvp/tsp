use super::{TspAlgorithm, TspState};
use crate::util;

pub struct BruteForce {
    cities: Vec<(f64, f64)>,
    perm: Vec<usize>,
    p_count: usize,
    length: f64,
    path: Vec<usize>,
}

impl TspAlgorithm for BruteForce {
    fn init(cities: Vec<(f64, f64)>) -> BruteForce {
        let path = (0..cities.len()).collect();
        Self {
            length: calculate_length(&cities, &path),
            perm: path.clone(),
            path,
            cities,
            p_count: 0,
        }
    }

    fn state(&self) -> TspState {
        TspState {
            length: self.length,
            path: self.path.clone(),
            sample: self.perm.clone(),
            status: format!("P: {}", self.p_count),
        }
    }

    fn step(&mut self) -> bool {
        if !next_permutation(&mut self.perm) {
            return true;
        }
        self.p_count += 1;
        let new_length = calculate_length(&self.cities, &self.path);
        if new_length < self.length {
            self.length = new_length;
            self.path = self.perm.clone();
        }
        return false;
    }
}

fn calculate_length(cities: &Vec<(f64, f64)>, path: &Vec<usize>) -> f64 {
    let mut length = 0.0;
    for i in 0..path.len() {
        let x = cities[path[i]];
        let y = cities[path[(i + 1) % path.len()]];
        length += util::dist(&x, &y);
    }
    length
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
        let h = perm[k];
        perm[k] = perm[l];
        perm[l] = h;
        // Reverse k+1 till end
        perm[k + 1..len].reverse();
        return true;
    }
    false
}
