fn main() {
    // GOAL:
    // Calculate the value of f(x) = x^2
    // for the range of x specified
    // [lower ; upper]

    let (lower, upper) = get_range();

    for i in lower..=upper {
        println!("f({i}) = {}", calculate(i));
    }
}

fn get_input() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error reading");
    input.trim().to_string()
}

fn get_range() -> (i32, i32) {
    println!("X's lower bound:");
    let l = get_input().parse().expect("Not a number");

    println!("X's upper bound:");
    let h = get_input().parse().expect("Not a number");

    (l, h)
}

fn calculate(x: i32) -> f64 {
    f64::from(x * x)
}
