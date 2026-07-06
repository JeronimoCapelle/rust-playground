fn main() {
    // Calculates an expression
    // by using two user provided numbers
    // and an operand.
    // Then displays it.

    let num_1 = get_number();
    let operand = get_operand();
    let num_2 = get_number();

    if !verify_operand(operand) {
        return;
    }

    let result = calculate(num_1, operand, num_2);

    display_result(num_1, operand, num_2, result);
}

fn get_number() -> i32 {
    println!("Input:");
    get_input().parse().expect("not a number")
}

fn get_operand() -> char {
    println!("Operand:");
    get_input().chars().next().expect("not a value")
}

fn get_input() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    input.trim().to_string()
}

fn display_result(x: i32, operand: char, y: i32, res: i32) {
    println!("-----------");
    println!("{x} {operand} {y} = {res}");
}

fn verify_operand(operand: char) -> bool {
    matches!(operand, '+' | '-' | '*' | '/')
}

fn calculate(x: i32, operand: char, y: i32) -> i32 {
    match operand {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => x / y,
        _ => 0,
    }
}
