use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    End,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Food {
    x: i32,
    y: i32,
}

impl Food {
    fn new() -> Self {
        let x = RandomNumberGenerator::new().range(0, SCREEN_WIDTH);
        let y = RandomNumberGenerator::new().range(0, SCREEN_HEIGHT);

        Food { x, y }
    }

    fn is_snake_collision(&mut self, snake: &Snake) -> bool {
        self.x == snake.x && self.y == snake.y
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, RED, BLACK, to_cp437('♥'))
    }
}

struct Snake {
    x: i32,
    y: i32,
    velocity: (f32, f32),
    speed: f32,
    body: Vec<(i32, i32, (f32, f32))>,
}

impl Snake {
    fn new(x: i32, y: i32, speed: f32) -> Self {
        Snake {
            x,
            y,
            velocity: (speed, 0.0),
            speed,
            body: Vec::from([
                (14, 25, (speed, 0.0)),
                (13, 25, (speed, 0.0)),
                (12, 25, (speed, 0.0)),
                (11, 25, (speed, 0.0)),
                (10, 25, (speed, 0.0)),
                (9, 25, (speed, 0.0)),
                (8, 25, (speed, 0.0)),
                (7, 25, (speed, 0.0)),
            ]),
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        self.velocity = match direction {
            Direction::Up => (0.0, -self.speed),
            Direction::Down => (0.0, self.speed),
            Direction::Right => (self.speed, 0.0),
            Direction::Left => (-self.speed, 0.0),
        };
    }

    fn is_wall_collision(&mut self) -> bool {
        self.x < 0 || self.x > SCREEN_WIDTH || self.y < 0 || self.y > SCREEN_HEIGHT
    }

    fn slither(&mut self) {
        for i in (0..self.body.len()).rev() {
            if i != 0 {
                self.body[i].0 = self.body[i - 1].0;
                self.body[i].1 = self.body[i - 1].1;
            } else {
                self.body[0].0 = self.x;
                self.body[0].1 = self.y;
            }
        }

        self.x += self.velocity.0 as i32;
        self.y += self.velocity.1 as i32;
    }

    fn grow(&mut self) {}

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'));
        self.body
            .iter()
            .for_each(|body_part| ctx.set(body_part.0, body_part.1, YELLOW, BLACK, to_cp437('■')))
    }
}

struct State {
    game_mode: GameMode,
    snake: Snake,
    food: Food,
    frame_time: f32,
    score: i32,
}

impl State {
    fn new() -> Self {
        Self {
            game_mode: GameMode::Menu,
            snake: Snake::new(15, 25, 1.0),
            food: Food::new(),
            frame_time: 0.0,
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to this unknown game");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.game_mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.snake = Snake::new(15, 25, 1.0)
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::W => self.snake.change_direction(Direction::Up),
                VirtualKeyCode::S => self.snake.change_direction(Direction::Down),
                VirtualKeyCode::D => self.snake.change_direction(Direction::Right),
                VirtualKeyCode::A => self.snake.change_direction(Direction::Left),
                VirtualKeyCode::Q => self.game_mode = GameMode::End,
                _ => {}
            }
        }

        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.snake.slither();
        }

        self.snake.render(ctx);

        self.food.render(ctx);

        if self.food.is_snake_collision(&self.snake) {
            self.score += 1;
            self.food = Food::new();
        }

        if self.snake.is_wall_collision() {
            self.game_mode = GameMode::End;
        }

        ctx.print(0, 0, &format!("Score: {}", self.score));
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        self.main_menu(ctx)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.game_mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Some Game")
        .build()?;

    main_loop(context, State::new())
}
