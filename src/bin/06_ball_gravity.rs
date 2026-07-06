const GRAVITY_ACCELERATION: f64 = 9.8;

fn main() {
    // GOAL:
    // Calculate the height of a ball as it free falls.
    // From 0s to 5s, starting from user provided height.

    let starting_height = get_input_number();

    for time in 0..=5 {
        let curr_height = calculate_height(starting_height, time);
        println!("{time}s = {}m", clamp_to_floor(curr_height));
    }
}

fn get_input_number() -> f64 {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    input.trim().parse().expect("not a number")
}

fn clamp_to_floor(h: f64) -> f64 {
    f64::clamp(h, 0.0, f64::MAX)
}

fn calculate_height(starting_height: f64, time: i32) -> f64 {
    starting_height - f64::from(time * time) * GRAVITY_ACCELERATION / 2.0
}
