const X_BORDER: i32 = 10;
const Y_BORDER: i32 = 10;

struct Ball {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
    radius: i32,
}

impl Ball {
    fn new(x: i32, y: i32, vx: i32, vy: i32, r: i32) -> Ball {
        Ball {
            x,
            y,
            velocity_x: vx,
            velocity_y: vy,
            radius: r,
        }
    }
    fn update_step(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }
    fn bounce(&mut self) {
        self.velocity_x = if self.x + self.radius > X_BORDER || self.x - self.radius < 0 {
            -self.velocity_x
        } else {
            self.velocity_x
        };
        self.velocity_y = if self.y + self.radius > Y_BORDER || self.y - self.radius < 0 {
            -self.velocity_y
        } else {
            self.velocity_y
        };
    }
}

fn main() {
    let mut ball = Ball::new(5, 5, 1, -1, 1);
    let mut i = 0;

    while i < 100 {
        ball.update_step();
        ball.bounce();
        i += 1;
    }
}
