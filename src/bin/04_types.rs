fn main() {
    println!("{}", describe(-1));
}

fn describe(n: i32) -> &'static str {
    match n.cmp(&0) {
        std::cmp::Ordering::Less => "negative",
        std::cmp::Ordering::Greater => "positive",
        std::cmp::Ordering::Equal => "zero",
    }
}
