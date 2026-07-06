fn main() {
    // GOAL:
    // ask the user for a temperature in Celsius,
    // convert it to Fahrenheit, and display both.

    let celsius = get_temp();
    let farenheit = convert_to_farenheit(celsius);
    display_both(celsius, farenheit);
}

fn get_temp() -> i32 {
    println!("Please provide a temperature in Celsius");
    get_input().parse().expect("not a number")
}

fn convert_to_farenheit(celsius: i32) -> i32 {
    (celsius * 9) / 5 + 32
}

fn display_both(celsius: i32, farenheit: i32) {
    println!("Celsius: {celsius}");
    println!("Farenheit: {farenheit}");
}

fn get_input() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Not a number");
    input.trim().to_string()
}
