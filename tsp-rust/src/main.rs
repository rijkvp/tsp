struct City {
    x: f64,
    y: f64,
}

fn dist_sqr(a: &City, b: &City) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    dx * dx + dy * dy
}

fn swap(arr: &mut [usize], x: usize, y: usize) {
    let h = arr[x];
    arr[x] = arr[y];
    arr[y] = h;
}

fn permute(arr: &mut [usize], res: &mut Vec<Vec<usize>>, p: usize) {
    if p == arr.len() {
        res.push(arr.to_vec());
    } else {
        for x in p..arr.len() {
            swap(arr, p, x);
            permute(arr, res, p + 1);
            swap(arr, p, x);
        }
    }
}

fn get_permutations(len: usize) -> Vec<Vec<usize>> {
    let mut arr = Vec::with_capacity(len);
    for i in 0..len {
        arr.push(i);
    }
    let mut res = Vec::new();
    permute(&mut arr, &mut res, 0);
    return res;
}

fn main() {
    let cities = vec![
        City { x: 250.0, y: 100.0 },
        City {
            x: 150.0,
            y: -200.0,
        },
        City {
            x: -100.0,
            y: 150.0,
        },
        City {
            x: -50.0,
            y: -100.0,
        },
    ];
    let mut shortest = f64::MAX;
    let mut path = None;
    for p in get_permutations(cities.len()) {
        let mut len = 0.0;
        for i in 1..p.len() {
            len += dist_sqr(&cities[p[i - 1]], &cities[p[i]]);
        }
        println!("Length of path {p:?}: {:.2}", len.sqrt());
        if len < shortest {
            shortest = len;
            path = Some(p);
        }
    }
    let len = shortest.sqrt();
    println!("Shortest path {:?} has a length of {len:.2}", path.unwrap());
}
