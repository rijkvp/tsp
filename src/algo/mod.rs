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

pub fn run<T: TspAlgorithm>(cities: Vec<(f64, f64)>) -> TspState {
    let mut algo = T::init(cities);
    while !algo.step() {}
    algo.state()
}
