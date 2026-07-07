use std::time::Duration;

fn main() {
    let initial_time = get_input();

    for i in 0..=initial_time {
        clear_terminal();
        display_number(initial_time - i);
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn get_input() -> i32 {
    let mut string = String::default();
    std::io::stdin()
        .read_line(&mut string)
        .expect("Error reading");
    string.trim().parse().expect("Not a number")
}

fn display_number(time: i32) {
    println!(",-------------------,");
    println!("|      +-----+      |");
    println!("|      |timer|      |");
    println!("|      +-----+      |");
    println!("|nununununununununun|");
    println!("|     ,-------.     |");
    println!("|  ,-'         `-.  |");
    println!("| /               \\ |");
    println!("|(       {time:^3}       )|");
    println!("| \\               / |");
    println!("|  `-.         ,-'  |");
    println!("|     `-------'     |");
    println!("'-------------------'");
}
