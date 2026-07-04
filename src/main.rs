use std::io;
fn main() {
    fn print_math(num_1: i32, num_2: i32) {
        println!("{num_1} + {num_2} is {}", num_1 + num_2);
        println!("{num_1} - {num_2} is {}", num_1 - num_2);
    }

    let mut input_1 = String::default();
    let mut input_2 = String::default();

    println!("Enter a whole number:");
    io::stdin().read_line(&mut input_1).expect("error reading");

    println!("Enter another whole number:");
    io::stdin().read_line(&mut input_2).expect("error reading");

    let num_1: i32 = input_1.trim().parse().expect("not a i32");
    let num_2: i32 = input_2.trim().parse().expect("not a i32");

    print_math(num_1, num_2);

    access_test();
}

fn access_test() {
    print_math(1, 2); //error
}
