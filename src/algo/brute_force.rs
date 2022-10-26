use super::TspAlgorithm;
use crate::util;

pub struct BruteForce {
    cities: Vec<(f64, f64)>,
    perm: Vec<usize>,
    p_count: usize,
    length: f64,
    path: Option<Vec<usize>>,
}

impl TspAlgorithm for BruteForce {
    fn init(cities: Vec<(f64, f64)>) -> BruteForce {
        Self {
            perm: (0..cities.len()).collect(),
            cities,
            p_count: 0,
            length: f64::MAX,
            path: None,
        }
    }

    fn state(&self) -> (f64, &Vec<usize>, String) {
        (
            self.length,
            self.path.as_ref().unwrap(),
            format!("P: {}", self.p_count),
        )
    }

    fn step(&mut self) -> bool {
        if !next_permutation(&mut self.perm) {
            return true;
        }
        self.p_count += 1;
        let mut new_length = 0.0;
        for i in 0..self.perm.len() {
            let x = self.cities[self.perm[i]];
            let y = self.cities[self.perm[(i + 1) % self.perm.len()]];
            new_length += util::dist(&x, &y);
        }
        if new_length < self.length {
            self.length = new_length;
            self.path = Some(self.perm.clone());
        }
        return false;
    }
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
