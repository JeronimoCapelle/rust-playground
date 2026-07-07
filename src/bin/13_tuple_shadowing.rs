fn main() {
    let name_1 = get_name("First person's name?");
    let name_1 = name_1.trim();

    let age_1: u8 = get_age("First person's age?");

    let name_2 = get_name("Second person's name?");
    let name_2 = name_2.trim();

    let age_2: u8 = get_age("Second person's age?");

    report(name_1, age_1, name_2, age_2);
}

fn get_name(question: &str) -> String {
    println!("{question}");
    get_input()
}

fn get_age(question: &str) -> u8 {
    println!("{question}");
    get_input().parse().expect("Not a valid age")
}

fn get_input() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading");
    input.trim().into()
}

fn report(name_1: &str, age_1: u8, name_2: &str, age_2: u8) {
    println!("{name_1:>6} {age_1:2}");
    println!("{name_2:>6} {age_2:2}");

    let comparison = match age_2.cmp(&age_1) {
        std::cmp::Ordering::Less => "is younger than",
        std::cmp::Ordering::Greater => "is older than",
        std::cmp::Ordering::Equal => "is the same age as",
    };

    println!("{name_2} {comparison} {name_1}");
}
