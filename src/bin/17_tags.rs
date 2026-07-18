fn main() {
    let who = ask("who is this for?");
    let what = ask("what is the gift?");
    let how_many = ask("how many?").parse().expect("not a number");

    let tag = make_tag(&who, &what, how_many);

    let (tag, len) = with_length(tag);

    println!("{tag}");
    println!("({len} bytes)");
}

fn ask(prompt: &str) -> String {
    let mut input = String::new();
    println!("{prompt}");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    input.trim().to_string()
}

fn make_tag(recipient: &str, gift: &str, count: u32) -> String {
    format!("To {recipient}: {count} x {gift}")
}

fn with_length(finished_tag: String) -> (String, usize) {
    let len = finished_tag.len();
    (finished_tag, len)
}
