use rand::Rng;
use crate::util;

pub fn run_annealing(cities: Vec<(f64, f64)>) {
    let n = cities.len();

    // 2D vector of the squared distances between cities; size n*n
    let mut distance: Vec<Vec<f64>> = vec![vec![-1.0; n]; n];

    for i in 0..n {
        for j in i..n {
            distance[i][j] = util::dist_sqr(&cities[i], &cities[j]);
            distance[j][i] = distance[i][j];
            if i != j {
                //the distance between the same two cities is always 0 and that's boring (don't print)
                println!(
                    "The distance between city #{} and city #{} is {:.2}.",
                    i, j, distance[i][j].sqrt()
                );
            }
        }
    }
    println!("Dist: {distance:#?}");

    // Initialize the path
    let mut path: Vec<usize> = vec![usize::MAX; n];
    for i in 0..n {
        path[i] = i as usize;
    }

    println!("Path: {path:?}");

    //by now, we have an initialized path and we have a 2D-vector holding the distances between
    //cities.

    let mut energy0 = calculate_energy(&path);
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

// Swap the elements of the indicies
fn swap_cities(path: &mut [usize]) {
    let (x, y) = distinct_indicies(path.len());
    util::swap(path, x, y);
}

// Invert the order of a random section
fn invert_order(path: &mut [usize]) {
    let (x, y) = distinct_indicies(path.len());
    todo!()
}

fn circular_shift_up(path: &mut [usize]) {
    let h = path[path.len() - 1];
    for i in 1..path.len() {
        path[i]  = path[i-1];
    }
    path[0] = h;
}

fn circular_shift_down(path: &mut [usize]) {
    let h = path[0];
    for i in 0..path.len()-1 {
        path[i]  = path[i+1];
    }
    path[path.len()-1] = h;
}

fn calculate_energy(path: &[usize]) -> f32 {
    todo!()
}
