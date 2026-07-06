const SCREEN_WIDTH: usize = 10;
const SCREEN_HEIGHT: usize = 10;
struct Screen {
    pixels: [[char; SCREEN_HEIGHT]; SCREEN_WIDTH],
}

impl Screen {
    fn new() -> Screen {
        Screen {
            pixels: [[' '; SCREEN_HEIGHT]; SCREEN_WIDTH],
        }
    }
    fn display(&self) {
        for i in self.pixels {
            for j in i {
                print!("{j}");
            }
            println!();
        }
    }
    fn clear(&mut self) {
        self.pixels = [[' '; SCREEN_HEIGHT]; SCREEN_WIDTH];
    }
}

fn main() {
    let mut screen = Screen::new();

    screen.display();
    screen.clear();
}
