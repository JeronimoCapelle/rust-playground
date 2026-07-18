fn main() {
    let (x, y, width, height) = get_inputs();
    display_rect(x, y, width, height);
}

fn get_inputs() -> (i32, i32, i32, i32) {
    let mut args = std::env::args().skip(1);
    (
        args.next()
            .expect("x not supplied")
            .parse::<i32>()
            .expect("Not a number"),
        args.next()
            .expect("y not supplied")
            .parse::<i32>()
            .expect("Not a number"),
        args.next()
            .expect("Width not supplied")
            .parse::<i32>()
            .expect("Not a number"),
        args.next()
            .expect("Height not supplied")
            .parse::<i32>()
            .expect("Not a number"),
    )
}

fn display_rect(x: i32, y: i32, width: i32, height: i32) {
    for _ in 0..y {
        for _ in 0..x + width {
            print!(" ");
        }
        println!();
    }

    for _ in 0..height {
        for _ in 0..x {
            print!(" ");
        }

        for _ in 0..width {
            print!("#");
        }
        println!();
    }
}
