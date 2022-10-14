use std::f32;
use rand::distributions::{Distribution, Uniform};
const N_MAX: usize = 50; //maximum number of cities
const N_MIN: usize = 2; //minimum number of cities

//calculate the energy (total length) of a path through all cities
fn calculate_energy(path: &[usize], distance_vector: &[Vec<f32>]) -> f32 {
    let mut sum: f32 = 0.0;
    for i in 0..=(path.len() - 2) { //calculate the sum of lengths of individual path segments
        sum += distance_vector[path[i]][path[i + 1]] as f32;
    }
    sum
}

fn random_number(a: usize, b: usize) -> usize{ //random integer between a and b (a incl, b excl)
    let mut rng = rand::thread_rng();
    let die = Uniform::from(a..b);
    die.sample(&mut rng)
}

//generates a string like "0 -> 2 -> 1 -> 3"
fn path_to_string(path: &[usize]) -> String {
    let mut path_string: String = path[0].to_string();
    for i in 1..path.len() {
        path_string.push_str(" -> ");
        path_string.push_str(&path[i].to_string());
    }
    path_string
}

fn swap_two_cities(path: &mut [usize], index1: usize, index2: usize) {
    let temp = (*path)[index1];
    (*path)[index1] = (*path)[index2];
    (*path)[index2] = temp;
}

fn main_aron() {
    println!("Hey there, welcome to the TSP program by Rijk van Putten and Aron Hardeman");
    //println!("Do you want to use Euclidean positions and distances (y/n)?");
    let mut inp = String::from("y"); //use euclidean positions by default

    //    std::io::stdin()
    //        .read_line(&mut inp)
    //        .expect("Woops, could not read your input.");
    //    inp = inp.trim().to_lowercase();
    let euclidean = {
        if inp == "y" || inp == "yes" {
            true
        } else if inp == "n" || inp == "no" {
            false
        } else {
            eprintln!("Woops, you didn\'t say y or n! Program will exit.");
            std::process::exit(1);
        }
    };

    println!(
        "How many cities would you like to have? {} <= n <= {}",
        N_MIN, N_MAX
    );

    inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Woops, could not read your input.");

    let n: usize = inp
        .trim()
        .parse()
        .expect("You should type a number, byebye");

    if n > N_MAX || n < N_MIN {
        eprintln!("Your input is not within the specified bounds.");
        std::process::exit(1);
    }

    //2D vector of distances between cities; size n*n
    let mut distance: Vec<Vec<f32>> = vec![vec![-1.0; n]; n];

    let mut path: Vec<usize> = vec![0; n];
    //initialize the path
    for i in 0..n {
        path[i] = i;
    }

    if euclidean {
        let mut position: Vec<(f32, f32)> = Vec::new(); //the x and y positions of the cities
                                                        //we use Pythagoras's theorem to calculate the distances in 2D Euclidean space
        for i in 0..n {
            println!(
                "Enter x and y locations of city #{} (separated by a space) and press enter:",
                i
            );
            inp = String::new();
            std::io::stdin().read_line(&mut inp).expect("Input failed");
            let current_position: Vec<f32> = inp
                .trim()
                .split(' ')
                .map(|x| x.parse().expect("input a number please"))
                .collect();
            position.push((current_position[0], current_position[1]));
        }
        assert!(position.len() == n);

        for i in 0..n {
            for j in i..n {
                let delta_x = position[i].0 - position[j].0;
                let delta_y = position[i].1 - position[j].1;
                distance[i][j] = f32::sqrt(delta_x * delta_x + delta_y * delta_y);
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
    } else {
        std::process::exit(1);
    }

    //by now, we have an initialized path and we have a 2D-vector holding the distances between
    //cities.
rand::distributions::{Distribution, Uniform};
    let mut energy0 = calculate_energy(&path, &distance);
    energy0 = calculate_energy(&path, &distance);
    println!("Initial path: {}", path_to_string(&path));
    println!("The energy (length) of the initially chosen path is {}.", energy0);
    
}
