fn main() {
    println!("First person's name?");

    let name_1 = {
        let mut input = String::default();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
        input
    };

    let name_1 = name_1.trim();

    println!("First person's age?");

    let age_1: u8 = {
        let mut input = String::default();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
        input.trim().parse().expect("Not a valid age")
    };

    println!("Second person's name?");

    let name_2 = {
        let mut input = String::default();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
        input
    };

    let name_2 = name_2.trim();

    println!("Second person's age?");

    let age_2: u8 = {
        let mut input = String::default();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
        input.trim().parse().expect("Not a valid age")
    };

    println!("{name_1:>6} {age_1:2}");
    println!("{name_2:>6} {age_2:2}");

    let comparison = match age_2.cmp(&age_1) {
        std::cmp::Ordering::Less => "is younger than",
        std::cmp::Ordering::Greater => "is older than",
        std::cmp::Ordering::Equal => "is the same age as",
    };

    println!("{name_2} {comparison} {name_1}");
}
