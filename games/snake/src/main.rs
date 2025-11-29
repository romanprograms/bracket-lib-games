use bracket_lib::prelude::*;

const SCREEN_WIDTH: u32 = 1200;
const SCREEN_HEIGHT: u32 = 800;
const TILE_W: u32 = 8;
const TILE_H: u32 = 8;
const SPRITE_TILE_SIZE: i32 = 40;
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

struct SnakePart {
    x: i32,
    y: i32,
    direction_vector: (i32, i32),
}

struct Snake {
    head_pos_x: i32,
    head_pos_y: i32,
    direction_unit_vector: (i32, i32),
    speed: i32,
    // speed: f32,
    
    snake_parts: Vec<SnakePart>,
    snake_tiles: SnakeParts,
    direction: Direction,
}



impl Snake {
    fn new(head_pos_x: i32, head_pos_y: i32) -> Self {
      let mut snake_parts = Vec::new();
      snake_parts.push(SnakePart {
          x: head_pos_x,
          y: head_pos_y,
          direction_vector: (1, 0),
      });

        Snake {
            head_pos_x,
            head_pos_y,
            snake_parts,
            direction_unit_vector: (1, 0),
            speed: 9,
            // speed: 1.0,
            snake_tiles: SnakeParts::new(),
            direction: Direction::Right,
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction_unit_vector = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        self.direction = direction;
        self.snake_parts[0].direction_vector = self.direction_unit_vector;
    }

    fn slither(&mut self) {
        self.head_pos_x += self.direction_unit_vector.0 * self.speed;
        self.head_pos_y += self.direction_unit_vector.1 * self.speed;
    }

    fn grow(&mut self) {
      // grow snake body logic here 
      if let Some(last_part) = self.snake_parts.last() {
          self.snake_parts.push(SnakePart {
              x: last_part.x - last_part.direction_vector.0 * self.speed,
              y: last_part.y - last_part.direction_vector.1 * self.speed,
              direction_vector: last_part.direction_vector,
          });
      }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // snake body will consist of pieces {x: i32,y: i32, direction_vector(i32,i32)}
        // each piece will have a sprite depending on direction vector and next piece direction vector
        // if no next piece, use head sprite depending on direction vector
        // if next piece has same direction vector, use straight body sprite 
        // if next piece has different direction vector, use corner body sprite depending on both direction vectors
        let sprite_index = match self.direction {
            Direction::Right => self.snake_tiles.head_right.index,
            Direction::Left => self.snake_tiles.head_left.index,
            Direction::Up => self.snake_tiles.head_up.index,
            Direction::Down => self.snake_tiles.head_down.index,
        };

        self.snake_parts.iter().skip(1).for_each(|part| {
            ctx.add_sprite(
                Rect::with_size(part.x , part.y , 40, 40),
                400,
                RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                // determine sprite index based on direction vector and next part direction vector
                match part.direction_vector {
                    (0, -1) | (0, 1) => self.snake_tiles.body_vertical.index,
                    (1, 0) | (-1, 0) => self.snake_tiles.body_horizontal.index,
                    _ => self.snake_tiles.body_horizontal.index, // placeholder for corner pieces
                },
            );
        });

        ctx.add_sprite(
            Rect::with_size(self.head_pos_x , self.head_pos_y , 40, 40),
            400,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            sprite_index, // self.frame % 4,
        );
    }
}

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
