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
        let mut random = RandomNumberGenerator::new();
        Food { x: random.range(0, SCREEN_WIDTH), y : random.range(0, SCREEN_HEIGHT)}
    }

    fn is_snake_collision(&mut self, snake: &Snake) -> bool {
        self.x == snake.head_pos_x && self.y == snake.head_pos_y
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, RED, BLACK, to_cp437('♥'))
    }
}

struct Snake {
    head_pos_x: i32,
    head_pos_y: i32,
    direction_unit_vector: (f32, f32),
    speed: f32,
    body_cells: Vec<(i32, i32)>,
    ghost_tail: (i32, i32),
}

impl Snake {
    fn new(head_pos_x: i32, head_pos_y: i32 ) -> Self {
        Snake {
            head_pos_x,
            head_pos_y,
            direction_unit_vector: (1.0, 0.0),
            speed: 1.0,
            body_cells: Vec::from([]),
            ghost_tail: (head_pos_x, head_pos_y),
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction_unit_vector = match direction {
            Direction::Up => (0.0, -1.0),
            Direction::Down => (0.0, 1.0),
            Direction::Right => (1.0, 0.0),
            Direction::Left => (-1.0, 0.0),
        };
    }

    fn is_wall_collision(&mut self) -> bool {
        self.head_pos_x < 0 || self.head_pos_x > SCREEN_WIDTH || self.head_pos_y < 0 || self.head_pos_y > SCREEN_HEIGHT
    }

    fn is_self_collision(&mut self) -> bool {
        match self
            .body_cells
            .iter()
            .find(|body_cell| body_cell.0 == self.head_pos_x && body_cell.1 == self.head_pos_y)
        {
            Some(_) => true,
            None => false,
        }
    }

    fn slither(&mut self) {
        if let Some(last) = self.body_cells.last() {
            self.ghost_tail = (last.0, last.1);
        }

        for i in (0..self.body_cells.len()).rev() {
            if i != 0 {
                self.body_cells[i].0 = self.body_cells[i - 1].0;
                self.body_cells[i].1 = self.body_cells[i - 1].1;
            } else {
                self.body_cells[0].0 = self.head_pos_x;
                self.body_cells[0].1 = self.head_pos_y;
            }
        }

        self.head_pos_x += (self.direction_unit_vector.0 * self.speed) as i32;
        self.head_pos_y += (self.direction_unit_vector.1 * self.speed) as i32;
    }

    fn grow(&mut self) {
        self.body_cells.push((self.ghost_tail.0, self.ghost_tail.1))
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.head_pos_x, self.head_pos_y, YELLOW, BLACK, to_cp437('@'));
        self.body_cells
            .iter()
            .for_each(|body_cell| ctx.set(body_cell.0, body_cell.1, YELLOW, BLACK, to_cp437('■')))
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
            snake: Snake::new(15, 25 ),
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
        self.snake = Snake::new(15, 25)
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
            self.snake.grow();
        }

        if self.snake.is_wall_collision() || self.snake.is_self_collision() {
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
        .with_title("Ascii Snake Game")
        .build()?;

    main_loop(context, State::new())
}
