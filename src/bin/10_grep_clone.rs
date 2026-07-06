fn main() {
    let pattern = "Hello";
    let contents = std::fs::read_to_string("input.txt").expect("failed to open");

    for i in contents.split('\n') {
        if i.contains(pattern) {
            println!("{i}");
        }
    }
}
