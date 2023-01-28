use std::collections::LinkedList;

use macroquad::prelude::*;

const SIZE: usize = 25;
const COLS: usize = 15;
const ROWS: usize = 15;
const WIDHT: usize = SIZE * ROWS;
const HEIGHT: usize = SIZE * COLS;

const SNAKE_COLOR: Color = BLUE;
const FOOD_COLOR: Color = RED;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        fullscreen: false,
        window_width: WIDHT as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut out = false;
    let mut score = 0;
    let mut snake = Snake::new();
    let mut food_position = vec2(COLS as f32 / 2., ROWS as f32 / 2.).floor();
    let mut timer = 0.;
    let mut update_time = 0.;
    const SPEED_TIME: f32 = 0.22;
    const MIN_SPEED_TIME: f32 = 0.1;
    let mut speed_time = SPEED_TIME;

    while !is_key_pressed(KeyCode::Escape) {
        if !out {
            timer += get_frame_time();

            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                snake.move_(Direction::Up)
            } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                snake.move_(Direction::Down)
            } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                snake.move_(Direction::Left)
            } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                snake.move_(Direction::Right)
            }

            if timer > update_time {
                update_time += speed_time;
                speed_time = speed_time.clamp(MIN_SPEED_TIME, SPEED_TIME);

                let random_position = || {
                    vec2(
                        rand::gen_range(0., ROWS as f32 - 1.),
                        rand::gen_range(0., COLS as f32 - 1.),
                    )
                    .floor()
                };

                // snake vs food
                if snake.head == food_position {
                    score += 1;
                    speed_time -= 0.005;
                    snake.increase_len();

                    while snake.contains_position(food_position) {
                        food_position = random_position();
                    }
                }

                snake.update(); // should be called afer increase len else collision to itself == true

                if snake.off_field(ROWS, COLS) || snake.collided_itself() {
                    out = true;
                }
            }
        } else {
            // out
            if is_key_pressed(KeyCode::Enter) {
                out = false;
                score = 0;
                snake = Snake::new();
                food_position = vec2(COLS as f32 / 2., ROWS as f32 / 2.).floor();
                timer = 0.;
                update_time = 0.;
                speed_time = SPEED_TIME;
            }
        }

        clear_background(WHITE);

        if !out {
            // grid
            for r in 0..ROWS {
                for c in 0..COLS {
                    draw_rectangle_lines(
                        (r * SIZE) as f32,
                        (c * SIZE) as f32,
                        SIZE as f32,
                        SIZE as f32,
                        3.,
                        GRAY,
                    )
                }
            }

            // food
            draw_rectangle(
                food_position.x * SIZE as f32,
                food_position.y * SIZE as f32,
                SIZE as f32,
                SIZE as f32,
                FOOD_COLOR,
            );

            // snake
            snake.draw();
        } else {
            // out
            const SCORE_FONT_SIZE: u16 = 40;
            let score_str = &format!("SCORE : {}", score);
            let score_measure = measure_text(score_str, None, FONT_SIZE, 1.);
            draw_text(
                score_str,
                screen_width() / 2. - score_measure.width,
                screen_height() * 1. / 3. - score_measure.height / 2.,
                SCORE_FONT_SIZE as f32,
                RED,
            );

            const FONT_SIZE: u16 = 26;
            let restart_str = "Press Enter to Restart.";
            let restart_measure = measure_text(restart_str, None, FONT_SIZE, 1.);
            draw_text(
                restart_str,
                screen_width() / 2. - restart_measure.width / 2.,
                screen_height() / 2. - restart_measure.height / 2.,
                FONT_SIZE as f32,
                RED,
            );
        }

        next_frame().await
    }

    println!("{}", score);
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const UP: Vec2 = vec2(0., -1.);
    const DOWN: Vec2 = vec2(0., 1.);
    const LEFT: Vec2 = vec2(-1., 0.);
    const RIGHT: Vec2 = vec2(1., 0.);

    pub fn get_vec(&self) -> Vec2 {
        match self {
            Direction::Up => Self::UP,
            Direction::Down => Self::DOWN,
            Direction::Left => Self::LEFT,
            Direction::Right => Self::RIGHT,
        }
    }
}

struct Snake {
    direction: Direction,
    head: Vec2,
    body: LinkedList<Vec2>,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            direction: Direction::Right,
            head: vec2(0., 0.),
            body: LinkedList::new(),
        }
    }

    pub fn increase_len(&mut self) {
        self.body.push_front(self.head);
    }

    pub fn move_(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                if self.direction != Direction::Down {
                    self.direction = Direction::Up;
                }
            }
            Direction::Down => {
                if self.direction != Direction::Up {
                    self.direction = Direction::Down;
                }
            }
            Direction::Left => {
                if self.direction != Direction::Right {
                    self.direction = Direction::Left;
                }
            }
            Direction::Right => {
                if self.direction != Direction::Left {
                    self.direction = Direction::Right;
                }
            }
        }
    }

    pub fn off_field(&self, rows: usize, cols: usize) -> bool {
        self.head.x < 0.
            || self.head.x >= rows as f32
            || self.head.y < 0.
            || self.head.y >= cols as f32
    }

    pub fn contains_position(&self, position: Vec2) -> bool {
        self.head == position || self.body.contains(&position)
    }

    pub fn collided_itself(&self) -> bool {
        self.body.contains(&self.head)
    }

    pub fn update(&mut self) {
        // self.body.push_front(self.head);
        // self.body.pop_back();
        // or

        let mut prev_head = self.head;
        for b in &mut self.body {
            let t = *b;
            *b = prev_head;
            prev_head = t;
        }

        self.head += self.direction.get_vec();
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.head.x * SIZE as f32,
            self.head.y * SIZE as f32,
            SIZE as f32,
            SIZE as f32,
            SNAKE_COLOR,
        );

        for b in &self.body {
            draw_rectangle(
                b.x * SIZE as f32,
                b.y * SIZE as f32,
                SIZE as f32,
                SIZE as f32,
                SNAKE_COLOR,
            );
        }
    }
}
