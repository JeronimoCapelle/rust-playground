use std::io;
fn main() {
    println!("Enter a whole number:");
    let input_1 = get_input();

    println!("Enter another whole number:");
    let input_2 = get_input();

    let num_1 = parse_num(&input_1);
    let num_2 = parse_num(&input_2);

    print_math(num_1, num_2);
}

fn print_math(num_1: i32, num_2: i32) {
    println!("{num_1} + {num_2} is {}", num_1 + num_2);
    println!("{num_1} - {num_2} is {}", num_1 - num_2);
}

fn parse_num(input: &str) -> i32 {
    input.trim().parse().expect("not a i32")
}

fn get_input() -> String {
    let mut input = String::default();
    io::stdin().read_line(&mut input).expect("error reading");
    input
}
