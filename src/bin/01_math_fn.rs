use std::io;

fn main() {
    // GOAL:
    // Take user input,
    // perform the two basic arithmetic operations,
    // and print the result

    println!("Enter a whole number:");
    let num_1 = get_num();

    println!("Enter another whole number:");
    let num_2 = get_num();

    print_math(num_1, num_2);
}

fn print_math(num_1: i32, num_2: i32) {
    println!("{num_1} + {num_2} is {}", num_1 + num_2);
    println!("{num_1} - {num_2} is {}", num_1 - num_2);
}

fn get_num() -> i32 {
    let mut input = String::default();
    io::stdin().read_line(&mut input).expect("error reading");
    input.trim().parse().expect("not a i32")
}
