fn main() {
    let x = get_number();
    let operand = get_operand();
    let y = get_number();

    let result = calculate(x, &operand, y);

    display_result(x, &operand, y, result);
}

fn get_number() -> i32 {
    println!("Input:");
    get_input().parse().expect("not a number")
}

fn get_operand() -> String {
    println!("Operand:");
    get_input()
}

fn get_input() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    input.trim().to_string()
}

fn display_result(x: i32, op: &String, y: i32, res: i32) {
    println!("{x} {op} {y} = {res}");
}

fn calculate(x: i32, op: &str, y: i32) -> i32 {
    match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        _ => 0,
    }
}
