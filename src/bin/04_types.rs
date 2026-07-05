fn main() {
    println!("{}", is_valid_percentage(0));
    println!("{}", is_valid_percentage(100));
    println!("{}", is_valid_percentage(101));
    println!("{}", is_valid_percentage(-1));
}

fn is_valid_percentage(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    x <= 100
}
