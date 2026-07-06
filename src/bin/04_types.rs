fn main() {
    let (min, max) = min_max(5, 3);
    println!("{min} < {max}");
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => (a, b),
        _ => (b, a),
    }
}
