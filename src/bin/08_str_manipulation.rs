fn main() {
    println!("What is your first name?");
    let first_name = get_input();

    println!("What is your last name?");
    let last_name = get_input();

    let name = build_name(&first_name, &last_name);

    println!("Your full name is:");
    println!("{name}");
}

fn build_name(first_name: &str, last_name: &str) -> String {
    format!("{first_name} {last_name}")
}

fn get_input() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    input.trim().to_string()
}
