use std::collections::LinkedList;

use macroquad::prelude::*;

const SIZE: usize = 25;
const COLS: usize = 12;
const ROWS: usize = 12;
const WIDHT: usize = SIZE * ROWS;
const HEIGHT: usize = SIZE * COLS;

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
    let mut snake = Snake::new();
    let mut timer = 0.;
    let mut update_time = 0.;
    let speed_time = 0.3;

    while !is_key_pressed(KeyCode::Escape) {
        if !out {
            timer += get_frame_time();

            if is_key_pressed(KeyCode::Up) {
                snake.move_(Direction::Up)
            } else if is_key_pressed(KeyCode::Down) {
                snake.move_(Direction::Down)
            } else if is_key_pressed(KeyCode::Left) {
                snake.move_(Direction::Left)
            } else if is_key_pressed(KeyCode::Right) {
                snake.move_(Direction::Right)
            }

            if timer > update_time {
                snake.update();
                update_time += speed_time;
            }

            if snake.off_field(ROWS, COLS) {
                out = true;
            }
        } else {
            // out
            if is_key_pressed(KeyCode::Enter) {
                out = false;
                snake = Snake::new();
            }
        }

        clear_background(BLACK);

        if !out {
            for r in 0..ROWS {
                for c in 0..COLS {
                    let color = if (r + c) % 2 == 0 { GRAY } else { BLACK };
                    draw_rectangle(
                        (r * SIZE) as f32,
                        (c * SIZE) as f32,
                        SIZE as f32,
                        SIZE as f32,
                        color,
                    )
                }
            }

            snake.draw();
        } else {
            // out
        }

        next_frame().await
    }
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

    pub fn update(&mut self) {
        self.head += self.direction.get_vec();
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.head.x * SIZE as f32,
            self.head.y * SIZE as f32,
            SIZE as f32,
            SIZE as f32,
            RED,
        );
    }
}
