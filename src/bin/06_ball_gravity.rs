fn main() {
    let height = get_number();

    for time in 0..=5 {
        let curr_height = calculate_height(height, time);
        println!("{time}s = {}m", clamp_floor(curr_height));
    }
}

fn get_number() -> f64 {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    input.trim().parse().expect("not a number")
}

fn clamp_floor(h: f64) -> f64 {
    f64::clamp(h, 0.0, f64::MAX)
}

fn calculate_height(height: f64, time: i32) -> f64 {
    height - f64::from(time * time * 49) / 10.0
}
