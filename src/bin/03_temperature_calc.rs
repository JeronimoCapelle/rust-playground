fn main() {
    // GOAL:
    // ask the user for a temperature in Celsius,
    // convert it to Fahrenheit, and display both.

    let celsius = get_temp();
    let farenheit = convert_to_farenheit(celsius);
    display_both(celsius, farenheit);
}

fn get_temp() -> f64 {
    println!("Please provide a temperature in Celsius");
    get_input().parse().expect("not a number")
}

fn convert_to_farenheit(celsius: f64) -> f64 {
    (celsius * 9.0) / 5.0 + 32.0
}

fn display_both(celsius: f64, farenheit: f64) {
    println!("Celsius: {celsius:.1}");
    println!("Farenheit: {farenheit:.1}");
}

fn get_input() -> String {
    let mut input = String::default();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Not a number");
    input.trim().to_string()
}
