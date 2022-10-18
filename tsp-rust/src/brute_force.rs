use crate::util;

pub struct BruteForce {
    cities: Vec<(f64, f64)>,
    permutations: Vec<Vec<usize>>,
    index: usize,
    length: f64,
    path: Option<Vec<usize>>,
}

impl BruteForce {
    pub fn new(cities: Vec<(f64, f64)>) -> BruteForce {
        let permutations = get_permutations(cities.len());
        Self {
            cities,
            permutations,
            index: 0,
            length: f64::MAX,
            path: None,
        }
    }

    pub fn run(mut self) -> (f64, Vec<usize>) {
        loop {
            if self.step() {
                return (self.length, self.path.unwrap());
            }
        }
    }

    pub fn step(&mut self) -> bool {
        let p = &self.permutations[self.index];
        let mut new_length = 0.0;
        for i in 0..p.len() {
            let x = self.cities[p[i]];
            let y = self.cities[p[(i + 1) % p.len()]];
            new_length += util::dist(&x, &y);
        }
        if new_length < self.length {
            self.length = new_length;
            self.path = Some(p.clone());
        }
        self.index += 1;
        self.index == self.permutations.len()
    }
}

fn get_permutations(len: usize) -> Vec<Vec<usize>> {
    let mut arr = Vec::with_capacity(len);
    for i in 0..len {
        arr.push(i);
    }
    let mut res = Vec::new();
    permute(&mut arr, &mut res, 0);
    return res;
}

fn permute(arr: &mut [usize], res: &mut Vec<Vec<usize>>, p: usize) {
    if p == arr.len() {
        res.push(arr.to_vec());
    } else {
        for x in p..arr.len() {
            util::swap(arr, p, x);
            permute(arr, res, p + 1);
            util::swap(arr, p, x);
        }
    }
}
