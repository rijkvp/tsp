use crate::util;
use rand::Rng;

pub fn run_annealing(cities: Vec<(f64, f64)>) {
    let n = cities.len();
    // 2D vector of the squared distances between cities; size n*n
    let mut distance: Vec<Vec<f64>> = vec![vec![-1.0; n]; n];

    for i in 0..n {
        for j in i..n {
            distance[i][j] = util::dist_sqr(&cities[i], &cities[j]);
            distance[j][i] = distance[i][j];
        }
    }

    // Initialize the path
    // TODO: Should we also randomize the path before starting?
    let mut path: Vec<usize> = vec![usize::MAX; n];
    for i in 0..n {
        path[i] = i as usize;
    }

    let mut rand = rand::thread_rng();
    let mut p = 0.5;
    let mut curr_path = path;
    let mut curr_dist = 0.0;
    for i in 1..n {
        let x = curr_path[i];
        curr_dist += distance[x - 1][x];
    }
    println!("Initial dist: {:.2}", curr_dist.sqrt());
    let mut i = 0;
    loop {
        // Quit after a certain amount of iterations
        if i > 200000 {
            break;
        }
        let new_path = match rand.gen_range(0..2) {
            0 => swap_cities(&curr_path),
            1 => invert_section(&curr_path),
            2 => shift(&curr_path),
            _ => panic!(),
        };
        let mut new_dist = 0.0;
        for i in 1..n {
            let x = new_path[i];
            let y = new_path[i - 1];
            if x > 0 {
                new_dist += distance[x][y];
            }
        }
        // Always accept a smaller path
        // Or accept a path with probability P
        if new_dist <= curr_dist || rand.gen_bool(p) {
            curr_path = new_path;
            curr_dist = new_dist;
        }
        println!(
            "{i} best: {curr_path:?}, dist: {:.2}, p: {p:.2}",
            curr_dist.sqrt()
        );
        // Decrease the probability of accepting a new path
        // TODO: Use the energy function
        p *= 0.99;
        i += 1;
    }
}

// Generate two distinct random numbers
fn distinct_indicies(max: usize) -> (usize, usize) {
    let mut rand = rand::thread_rng();
    let x = rand.gen_range(0..max);
    let mut y = x;
    while y == x {
        y = rand.gen_range(0..max);
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
    let mut rand = rand::thread_rng();
    if rand.gen_bool(0.5) {
        util::shift_right(path)
    } else {
        util::shift_left(path)
    }
}
