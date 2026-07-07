fn main() {
    let total = 42.5;
    let formatted = format!("Total: ${total:>7.2}");
    println!("{formatted}");
}
