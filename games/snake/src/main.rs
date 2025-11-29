use bracket_lib::prelude::*;

const SCREEN_WIDTH: u32 = 1200;
const SCREEN_HEIGHT: u32 = 800;
const TILE_W: u32 = 8;
const TILE_H: u32 = 8;
const SPRITE_TILE_SIZE: i32 = 40;
const DEFAULT_SNAKE_LENGTH: i32 = 5;
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

struct Food {
    pos_x: i32,
    pos_y: i32,
}

impl Food {
    fn new() -> Self {
        // Self { pos_x: 0, pos_y: 0 }
        Self {
            pos_x: RandomNumberGenerator::new().range(0, SCREEN_WIDTH as i32 - SPRITE_TILE_SIZE),
            pos_y: RandomNumberGenerator::new().range(0, SCREEN_HEIGHT as i32 - SPRITE_TILE_SIZE),
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.add_sprite(
            Rect::with_size(self.pos_x as i32, self.pos_y as i32, 40, 40),
            400,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            14, // self.frame % 4,
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
    direction_vector: DirectionVector,
}

struct Snake {
    head_pos_x: i32,
    head_pos_y: i32,
    speed: i32,
    snake_parts: Vec<SnakePart>,
    snake_tiles: SnakeParts,
}

impl Snake {
    fn new(head_pos_x: i32, head_pos_y: i32) -> Self {
        Snake {
            head_pos_x,
            head_pos_y,
            snake_parts: (0..DEFAULT_SNAKE_LENGTH)
                .map(|i| SnakePart {
                    x: head_pos_x - i * 40,
                    y: head_pos_y,
                    direction_vector: DirectionVector { x: 1, y: 0 },
                })
                .collect(),
            speed: 5,
            snake_tiles: SnakeParts::new(),
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        self.snake_parts[0].direction_vector = match direction {
            Direction::Up => DirectionVector { x: 0, y: -1 },
            Direction::Down => DirectionVector { x: 0, y: 1 },
            Direction::Left => DirectionVector { x: -1, y: 0 },
            Direction::Right => DirectionVector { x: 1, y: 0 },
        };
    }

    fn slither(&mut self) {
        for (index) in (0..self.snake_parts.len()).rev() {
            if index != 0 {
                self.snake_parts[index].direction_vector.x =
                    self.snake_parts[index - 1].direction_vector.x;
                self.snake_parts[index].direction_vector.y =
                    self.snake_parts[index - 1].direction_vector.y;
            }
        }

        self.snake_parts.iter_mut().for_each(|part| {
            part.x += part.direction_vector.x * self.speed;
            part.y += part.direction_vector.y * self.speed;
        });

        // if let Some(head) = self.snake_parts.first() {
        //     self.snake_parts[0].x += self.direction_unit_vector.0 * self.speed;
        //     self.snake_parts[0].y += self.direction_unit_vector.1 * self.speed;
        // }
    }

    fn grow(&mut self) {}

    fn render(&mut self, ctx: &mut BTerm) {
        // snake body will consist of pieces {x: i32,y: i32, direction_vector(i32,i32)}
        // each piece will have a sprite depending on direction vector and next piece direction vector
        // if no next piece, use head sprite depending on direction vector
        // if next piece has same direction vector, use straight body sprite
        // if next piece has different direction vector, use corner body sprite depending on both direction vectors
        // let sprite_index = match self.direction {
        //     Direction::Right => self.snake_tiles.head_right.index,
        //     Direction::Left => self.snake_tiles.head_left.index,
        //     Direction::Up => self.snake_tiles.head_up.index,
        //     Direction::Down => self.snake_tiles.head_down.index,
        // };
        //
        for (index, part) in self.snake_parts.iter().enumerate().rev() {
            let is_tail = index == self.snake_parts.len() - 1;
            let is_head = index == 0;

            if is_head {
                let sprite_index = match part.direction_vector {
                    DirectionVector { x: 1, y: 0 } => self.snake_tiles.head_right.index,
                    DirectionVector { x: -1, y: 0 } => self.snake_tiles.head_left.index,
                    DirectionVector { x: 0, y: -1 } => self.snake_tiles.head_up.index,
                    DirectionVector { x: 0, y: 1 } => self.snake_tiles.head_down.index,
                    _ => 0,
                };
                ctx.add_sprite(
                    Rect::with_size(part.x, part.y, 40, 40),
                    400,
                    RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                    // determine sprite index based on direction vector and next part direction vector
                    sprite_index,
                );
            } else {
                ctx.add_sprite(
                    Rect::with_size(part.x, part.y, 40, 40),
                    400,
                    RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                    // determine sprite index based on direction vector and next part direction vector
                    1,
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
            snake: Snake::new(300, 100),
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
                _ => {}
            }
        }

        self.timer += ctx.frame_time_ms;

        if self.timer > 66.0 {
            self.timer = 0.0;
            self.frame += 1;

            if rects_overlap(
                self.snake.head_pos_x,
                self.snake.head_pos_y,
                SPRITE_TILE_SIZE,
                SPRITE_TILE_SIZE,
                self.food.pos_x,
                self.food.pos_y,
                SPRITE_TILE_SIZE,
                SPRITE_TILE_SIZE,
            ) {
                self.food = Food::new();
                // self.snake.speed += 2;
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
                .add_sprite(Rect::with_size(0, snake_tile_map.tail_up.pos_y, 40, 40))
                .add_sprite(Rect::with_size(0, snake_tile_map.tail_right.pos_y, 40, 40))
                .add_sprite(Rect::with_size(0, snake_tile_map.tail_left.pos_y, 40, 40))
                .add_sprite(Rect::with_size(0, snake_tile_map.tail_down.pos_y, 40, 40))
                .add_sprite(Rect::with_size(0, snake_tile_map.head_up.pos_y, 40, 40))
                .add_sprite(Rect::with_size(0, snake_tile_map.head_right.pos_y, 40, 40))
                .add_sprite(Rect::with_size(0, snake_tile_map.head_left.pos_y, 40, 40))
                .add_sprite(Rect::with_size(0, snake_tile_map.head_down.pos_y, 40, 40))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_vertical.pos_y,
                    40,
                    40,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_topright.pos_y,
                    40,
                    40,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_topleft.pos_y,
                    40,
                    40,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_horizontal.pos_y,
                    40,
                    40,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_bottomright.pos_y,
                    40,
                    40,
                ))
                .add_sprite(Rect::with_size(
                    0,
                    snake_tile_map.body_bottomleft.pos_y,
                    40,
                    40,
                ))
                .add_sprite(Rect::with_size(0, 560, 40, 40)),
        )
        .with_vsync(false)
        .build()?;

    main_loop(context, State::new())
}
