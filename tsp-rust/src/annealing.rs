use crate::util::dist_sqr;

pub fn run_annealing(cities: Vec<(f64, f64)>) {
    let n = cities.len();

    //2D vector of distances between cities; size n*n
    let mut distance: Vec<Vec<f64>> = vec![vec![-1.0; n]; n];

    let mut path: Vec<i32> = vec![-1; n];
    //initialize the path
    for i in 0..n {
        path[i] = i as i32;
    }

    for i in 0..n {
        for j in i..n {
            distance[i][j] = dist_sqr(&cities[i], &cities[j]);
            distance[j][i] = distance[i][j];
            if i != j {
                //the distance between the same two cities is always 0 and that's boring (don't print)
                println!(
                    "The distance between city #{} and city #{} is {}.",
                    i, j, distance[i][j]
                );
            }
        }
    }

    //by now, we have an initialized path and we have a 2D-vector holding the distances between
    //cities.

    let mut energy0 = calculate_energy(path);
}

fn calculate_energy(path: Vec<i32>) -> f32 {
    todo!()
}