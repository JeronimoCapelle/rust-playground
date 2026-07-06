fn main() {
    //GOAL:
    // Upgrade from the previous simple calc.
    // This one takes an operand as well

    let x = get_number();
    let y = get_number();
    let op = get_operand();
    let result = calculate(x, y, op);
    display_result(x, op, y, result);
}

fn get_input() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading");
    input.trim().to_string()
}

fn get_number() -> f64 {
    get_input().parse().expect("not a number")
}

fn get_operand() -> char {
    get_input().chars().next().expect("char reading failed")
}

fn calculate(x: f64, y: f64, operand: char) -> f64 {
    match operand {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => x / y,
        _ => todo!(),
    }
}

fn display_result(x: f64, op: char, y: f64, res: f64) {
    println!("{x} {op} {y} = {res}");
}
