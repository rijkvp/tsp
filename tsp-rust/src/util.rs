pub fn dist_sqr(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    dx * dx + dy * dy
}