/// Calculate the squared distance between two points
pub fn dist_sqr(a: &(f64, f64), b: &(f64, f64)) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    dx * dx + dy * dy
}

/// Swaps two elements of an array
pub fn swap(arr: &mut [usize], x: usize, y: usize) {
    let h = arr[x];
    arr[x] = arr[y];
    arr[y] = h;
}
