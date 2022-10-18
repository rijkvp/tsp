pub mod annealing;
pub mod brute_force;

pub trait TspAlgorithm {
    fn init(cities: Vec<(f64, f64)>) -> Self;
    fn step(&mut self) -> bool;
    fn state(&self) -> (f64, &Vec<usize>, String);
}
