pub mod annealing;
pub mod brute_force;

pub struct TspState {
    pub length: f64,
    pub path: Vec<usize>,
    pub sample: Vec<usize>,
    pub status: String,
}

pub trait TspAlgorithm {
    fn init(cities: Vec<(f64, f64)>) -> Self;
    fn step(&mut self) -> bool;
    fn state(&self) -> TspState;
}
