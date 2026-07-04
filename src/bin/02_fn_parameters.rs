fn main() {
    print!("{}", adder(2, adder(1, 4)));
}

fn adder(x: i32, y: i32) -> i32 {
    x + y
}
