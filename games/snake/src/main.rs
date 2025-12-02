use std::unreachable;

use bracket_lib::prelude::*;

const SCREEN_WIDTH: u32 = 1800;
const SCREEN_HEIGHT: u32 = 1200;
const TILE_W: u32 = 8;
const TILE_H: u32 = 8;
const SPRITE_TILE_SIZE: i32 = 40;
const DEFAULT_SNAKE_LENGTH: i32 = 3;
// TODO:
// const FRAME_DURATION: f32 = 75.0;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn rects_overlap(ax: i32, ay: i32, aw: i32, ah: i32, bx: i32, by: i32, bw: i32, bh: i32) -> bool {
    !(ax + aw <= bx  ||  // A is completely left of B
      ax >= bx + bw  ||  // A is completely right of B
      ay + ah <= by  ||  // A is completely above B
      ay >= by + bh) // A is completely below B
}

fn direction_between(a: &SnakePart, b: &SnakePart) -> Direction {
    match (b.x.cmp(&a.x), b.y.cmp(&a.y)) {
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Direction::Right,
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Direction::Left,

        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Direction::Down,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Direction::Up,
        _ => unreachable!("Won't reach this code"),
    }
}

struct Food {
    pos_x: i32,
    pos_y: i32,
}

impl Food {
    fn new() -> Self {
        Self {
            pos_x: RandomNumberGenerator::new().range(0, SCREEN_WIDTH as i32 - SPRITE_TILE_SIZE),
            pos_y: RandomNumberGenerator::new().range(0, SCREEN_HEIGHT as i32 - SPRITE_TILE_SIZE),
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.add_sprite(
            Rect::with_size(
                self.pos_x as i32,
                self.pos_y as i32,
                SPRITE_TILE_SIZE,
                SPRITE_TILE_SIZE,
            ),
            400,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            14,
        );
    }
}

struct DirectionVector {
    x: i32,
    y: i32,
}

struct SnakePart {
    x: i32,
    y: i32,
}

struct Snake {
    speed: i32,
    direction_vector: DirectionVector,
    snake_parts: Vec<SnakePart>,
    snake_tiles: SnakeParts,
}

impl Snake {
    fn new(head_pos_x: i32, head_pos_y: i32) -> Self {
        Snake {
            snake_parts: (0..DEFAULT_SNAKE_LENGTH)
                .map(|i| SnakePart {
                    x: head_pos_x - i * SPRITE_TILE_SIZE,
                    y: head_pos_y,
                })
                .collect(),
            direction_vector: DirectionVector { x: 1, y: 0 },
            speed: SPRITE_TILE_SIZE,
            snake_tiles: SnakeParts::new(),
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction_vector = match direction {
            Direction::Up => DirectionVector { x: 0, y: -1 },
            Direction::Down => DirectionVector { x: 0, y: 1 },
            Direction::Left => DirectionVector { x: -1, y: 0 },
            Direction::Right => DirectionVector { x: 1, y: 0 },
        };
    }

    fn slither(&mut self) {
        if self.speed == 0 {
            return;
        }

        let new_head = SnakePart {
            x: self.snake_parts[0].x + self.direction_vector.x * self.speed,
            y: self.snake_parts[0].y + self.direction_vector.y * self.speed,
        };

        self.snake_parts.insert(0, new_head);
        self.snake_parts.pop();
    }

    fn grow(&mut self) {
        let new_head = SnakePart {
            x: self.snake_parts[0].x + self.direction_vector.x * self.speed,
            y: self.snake_parts[0].y + self.direction_vector.y * self.speed,
        };

        self.snake_parts.insert(0, new_head);
    }

    fn render(&mut self, ctx: &mut BTerm) {
        for (index, part) in self.snake_parts.iter().enumerate() {
            let is_tail = index == self.snake_parts.len() - 1;
            let is_head = index == 0;

            if (is_head) {
                let sprite_index = match self.direction_vector {
                    DirectionVector { x: 1, y: 0 } => self.snake_tiles.head_right.index,
                    DirectionVector { x: 0, y: 1 } => self.snake_tiles.head_down.index,
                    DirectionVector { x: -1, y: 0 } => self.snake_tiles.head_left.index,
                    DirectionVector { x: 0, y: -1 } => self.snake_tiles.head_up.index,
                    _ => 14,
                };

                ctx.add_sprite(
                    Rect::with_size(part.x, part.y, SPRITE_TILE_SIZE, SPRITE_TILE_SIZE),
                    400,
                    RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                    sprite_index,
                );
            } else if (is_tail) {
                let next_part = &self.snake_parts[index - 1];

                let sprite_index = match (next_part.x.cmp(&part.x), next_part.y.cmp(&part.y)) {
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                        self.snake_tiles.tail_left.index
                    }

                    (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                        self.snake_tiles.tail_right.index
                    }

                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                        self.snake_tiles.tail_up.index
                    }
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                        self.snake_tiles.tail_down.index
                    }
                    _ => 14,
                };

                ctx.add_sprite(
                    Rect::with_size(part.x, part.y, SPRITE_TILE_SIZE, SPRITE_TILE_SIZE),
                    400,
                    RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                    sprite_index,
                );
            } else {
                let next = &self.snake_parts[index - 1];
                let prev = &self.snake_parts[index + 1];

                let from_prev = direction_between(prev, part);
                let to_next = direction_between(part, next);

                let sprite_index = match (from_prev, to_next) {
                    (Direction::Up, Direction::Down)
                    | (Direction::Down, Direction::Up)
                    | (Direction::Up, Direction::Up)
                    | (Direction::Down, Direction::Down) => self.snake_tiles.body_vertical.index,

                    (Direction::Left, Direction::Right)
                    | (Direction::Right, Direction::Left)
                    | (Direction::Right, Direction::Right)
                    | (Direction::Left, Direction::Left) => self.snake_tiles.body_horizontal.index,

                    // Corners
                    (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                        self.snake_tiles.body_bottomright.index
                    }

                    (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
                        self.snake_tiles.body_bottomleft.index
                    }

                    (Direction::Right, Direction::Up) | (Direction::Down, Direction::Left) => {
                        self.snake_tiles.body_topleft.index
                    }

                    (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                        self.snake_tiles.body_topright.index
                    }

                    _ => unreachable!("straight segments already handled"),
                };

                ctx.add_sprite(
                    Rect::with_size(part.x, part.y, 40, 40),
                    400,
                    RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                    sprite_index,
                );
            }
        }
    }
}

//  TODO:
// enum GameMode {
//   Playing,
//   End,
//   Menu
//   Pause
// }

struct State {
    food: Food,
    snake: Snake,
    frame: usize,
    timer: f32,
}

impl State {
    fn new() -> Self {
        Self {
            food: Food::new(),
            snake: Snake::new(280, 80),
            frame: 0,
            timer: 0.0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.print(1, 1, "Snake Game!");
        ctx.printer(
            70,
            1,
            &format!("#[pink]FPS: #[]{}", ctx.fps),
            TextAlign::Left,
            None,
        );

        ctx.set_active_console(0);
        ctx.cls();

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::W => self.snake.change_direction(Direction::Up),
                VirtualKeyCode::S => self.snake.change_direction(Direction::Down),
                VirtualKeyCode::A => self.snake.change_direction(Direction::Left),
                VirtualKeyCode::D => self.snake.change_direction(Direction::Right),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }

        self.timer += ctx.frame_time_ms;

        if self.timer > 66.0 {
            self.timer = 0.0;
            self.frame += 1;

            if rects_overlap(
                self.snake.snake_parts[0].x,
                self.snake.snake_parts[0].y,
                SPRITE_TILE_SIZE,
                SPRITE_TILE_SIZE,
                self.food.pos_x,
                self.food.pos_y,
                SPRITE_TILE_SIZE,
                SPRITE_TILE_SIZE,
            ) {
                self.food = Food::new();
                self.snake.grow();
            }
            // snake move
            self.snake.slither();
        }
        self.food.render(ctx);
        self.snake.render(ctx);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.play(ctx);
    }
}

embedded_resource!(NYAN_CAT, "../resources/texture.png");

struct SnakePartSpriteLocation {
    index: usize,
    pos_y: i32,
}

struct SnakeParts {
    tail_up: SnakePartSpriteLocation,
    tail_right: SnakePartSpriteLocation,
    tail_left: SnakePartSpriteLocation,
    tail_down: SnakePartSpriteLocation,
    head_up: SnakePartSpriteLocation,
    head_right: SnakePartSpriteLocation,
    head_left: SnakePartSpriteLocation,
    head_down: SnakePartSpriteLocation,
    body_vertical: SnakePartSpriteLocation,
    body_topright: SnakePartSpriteLocation,
    body_topleft: SnakePartSpriteLocation,
    body_horizontal: SnakePartSpriteLocation,
    body_bottomright: SnakePartSpriteLocation,
    body_bottomleft: SnakePartSpriteLocation,
}

impl SnakeParts {
    fn new() -> Self {
        Self {
            tail_up: SnakePartSpriteLocation { index: 0, pos_y: 0 },
            tail_right: SnakePartSpriteLocation {
                index: 1,
                pos_y: 40,
            },
            tail_left: SnakePartSpriteLocation {
                index: 2,
                pos_y: 80,
            },
            tail_down: SnakePartSpriteLocation {
                index: 3,
                pos_y: 120,
            },
            head_up: SnakePartSpriteLocation {
                index: 4,
                pos_y: 160,
            },
            head_right: SnakePartSpriteLocation {
                index: 5,
                pos_y: 200,
            },
            head_left: SnakePartSpriteLocation {
                index: 6,
                pos_y: 240,
            },
            head_down: SnakePartSpriteLocation {
                index: 7,
                pos_y: 280,
            },
            body_vertical: SnakePartSpriteLocation {
                index: 8,
                pos_y: 320,
            },
            body_topright: SnakePartSpriteLocation {
                index: 9,
                pos_y: 360,
            },
            body_topleft: SnakePartSpriteLocation {
                index: 10,
                pos_y: 400,
            },
            body_horizontal: SnakePartSpriteLocation {
                index: 11,
                pos_y: 440,
            },
            body_bottomright: SnakePartSpriteLocation {
                index: 12,
                pos_y: 480,
            },
            body_bottomleft: SnakePartSpriteLocation {
                index: 13,
                pos_y: 520,
            },
        }
    }
}

fn main() -> BError {
    link_resource!(NYAN_CAT, "resources/texture.png");
    println!("snake with sprites");

    let snake_tile_map = SnakeParts::new();

    let context = BTermBuilder::new()
        .with_title("Sprite Snake")
        .with_dimensions(SCREEN_WIDTH / TILE_W, SCREEN_HEIGHT / TILE_H)
        .with_tile_dimensions(TILE_W, TILE_H)
        .with_font("terminal8x8.png", 8, 8)
        .with_sprite_console(SCREEN_WIDTH, SCREEN_HEIGHT, 0)
        .with_simple_console_no_bg(80, 50, "terminal8x8.png")
        .with_sprite_sheet(
            SpriteSheet::new("resources/texture.png")
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.tail_up.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.tail_right.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.tail_left.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.tail_down.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.head_up.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.head_right.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.head_left.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.head_down.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_vertical.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_topright.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_topleft.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_horizontal.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_bottomright.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_bottomleft.pos_y,
                    SPRITE_TILE_SIZE,
                    SPRITE_TILE_SIZE,
                ))
                .add_sprite(Rect::with_size(0, 560, SPRITE_TILE_SIZE, SPRITE_TILE_SIZE)),
        )
        .with_vsync(false)
        .build()?;

    main_loop(context, State::new())
}
