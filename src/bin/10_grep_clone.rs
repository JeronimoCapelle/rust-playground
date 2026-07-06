fn main() {
    let inputs: Vec<_> = std::env::args().collect();
    let contents = std::fs::read_to_string(&inputs[1]).expect("failed to open");

    for i in contents.split('\n') {
        if i.contains(&inputs[2]) {
            println!("{i}");
        }
    }
}
