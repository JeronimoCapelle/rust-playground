const LOADING_BAR_SIZE: usize = 20;
const SECONDS_PER_MINUTE: usize = 60;
const FILLED_BAR: char = '#';

fn main() {
    let initial_time: usize = std::env::args()
        .nth(1)
        .expect("The first arg should be time in seconds")
        .parse()
        .expect("Not a number");

    for i in 0..=initial_time {
        clear_terminal();
        display_time(initial_time, initial_time - i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn display_time(initial_time: usize, time: usize) {
    let mut bar = String::new();

    let size = (initial_time - time) * LOADING_BAR_SIZE / initial_time;
    for _ in 0..size {
        bar.push(FILLED_BAR);
    }

    dbg!(&bar);

    println!(
        "[{bar:<LOADING_BAR_SIZE$}] {:02}:{:02}",
        time / SECONDS_PER_MINUTE,
        time % SECONDS_PER_MINUTE
    );
}
