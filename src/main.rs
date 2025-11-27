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

struct State {
    game_mode: GameMode,
    player: Player,
    frame_time: f32,
}

struct Player {
    x: i32,
    y: i32,
    velocity: (f32, f32),
    speed: f32,
}

struct Food {
    x: i32,
    y: i32,
}

impl Food {
    fn new(x: i32, y: i32) -> Self {
        Food { x, y }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, YELLOW, CYAN, to_cp437('☺'))
    }
}

impl Player {
    fn new(x: i32, y: i32, speed: f32) -> Self {
        Player {
            x,
            y,
            velocity: (speed, 0.0),
            speed,
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
        self.x += self.velocity.0 as i32;
        self.y += self.velocity.1 as i32;
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'));
    }
}

impl State {
    fn new() -> Self {
        Self {
            game_mode: GameMode::Menu,
            player: Player::new(15, 25, 1.0),
            frame_time: 0.0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
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
        self.player = Player::new(15, 25, 1.0)
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::W => self.player.change_direction(Direction::Up),
                VirtualKeyCode::S => self.player.change_direction(Direction::Down),
                VirtualKeyCode::D => self.player.change_direction(Direction::Right),
                VirtualKeyCode::A => self.player.change_direction(Direction::Left),
                VirtualKeyCode::Q => self.game_mode = GameMode::End,
                _ => {}
            }
        }

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.player.slither();
        }

        self.player.render(ctx);

        if self.player.is_wall_collision() {
            self.game_mode = GameMode::End;
        }
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
        .with_title("Ascii Snake")
        .build()?;

    main_loop(context, State::new())
}
