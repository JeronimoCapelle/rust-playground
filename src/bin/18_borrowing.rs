fn main() {
    let input = get_input();

    let first_word = first_word(&input);
    println!("first word: {first_word}");

    let shout = &mut shouted(first_word);
    add_excitement(shout);
    println!("{shout}");
}

fn get_input() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    input.trim().to_string()
}

fn first_word(s: &str) -> &str {
    match s.find(" ") {
        Some(a) => &s[..a],
        None => s,
    }
}

fn shouted(text: &str) -> String {
    text.to_uppercase()
}

fn add_excitement(text: &mut String) {
    text.push_str("!!!");
}
