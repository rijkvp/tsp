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

/// Returns a new vector with all elements circularly shifted to the right
pub fn shift_right(arr: &[usize]) -> Vec<usize> {
    let mut res = Vec::with_capacity(arr.len());
    res.push(arr[arr.len() - 1]);
    for i in 0..arr.len() - 1 {
        res.push(arr[i]);
    }
    res
}

/// Returns a new vector with all elements circularly shifted to the left
pub fn shift_left(arr: &[usize]) -> Vec<usize> {
    let mut res = Vec::with_capacity(arr.len());
    for i in 1..arr.len() {
        res.push(arr[i]);
    }
    res.push(arr[0]);
    res
}

#[cfg(test)]
mod test {
    #[test]
    fn test_swap() {
        let mut arr1 = [1, 2, 3, 4];
        super::swap(&mut arr1, 1, 2);
        assert_eq!(arr1, [1, 3, 2, 4]);
    }

    #[test]
    fn test_shift() {
        let arr1 = [1, 2, 3, 4];
        let arr2 = super::shift_right(&arr1);
        assert_eq!(arr2, [4, 1, 2, 3]);
        let arr3 = super::shift_left(&arr2);
        assert_eq!(arr3, arr1);
    }
}
