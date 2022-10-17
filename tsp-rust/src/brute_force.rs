use crate::util;

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

fn get_permutations(len: usize) -> Vec<Vec<usize>> {
    let mut arr = Vec::with_capacity(len);
    for i in 0..len {
        arr.push(i);
    }
    let mut res = Vec::new();
    permute(&mut arr, &mut res, 0);
    return res;
}

pub fn run_brute_force(cities: Vec<(f64, f64)>) -> (f64, Vec<usize>) {
    let mut shortest = f64::MAX;
    let mut path = None;
    let permutations = get_permutations(cities.len());
    for p in permutations {
        let mut len = 0.0;
        for i in 0..p.len() {
            let x = cities[p[i]];
            let y = cities[p[(i+1) % p.len()]];
            len += util::dist(&x, &y);
        }
        if len < shortest {
            shortest = len;
            path = Some(p);
        }
    }
    (shortest, path.unwrap())
}
