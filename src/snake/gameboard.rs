use std::{ops::Mul, mem::swap};

use js_sys::Math::random;

use super::{food::Food, pos::Pos, snake::Snake};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct GameBoard<'a> {
    pub width: u32,
    pub height: u32,
    pub snake: Snake<'a>,
    pub is_failed: bool,
    pub food: Food<'a>,
}

impl GameBoard<'_> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            snake: Snake::new(
                (0..3)
                    .map(|n| Pos(width as i32 / 2 + n, height as i32 / 2))
                    .collect(),
            ),
            is_failed: false,
            food: Food {
                pos: Pos(
                    random().mul(width as f64).floor() as i32,
                    random().mul(height as f64).floor() as i32
                ),
                food_color: vec!["🟥", "🟧", "🟨", "🟩", "🟦", "🟪", "🟫"].get(random().mul(7.0).floor() as usize).unwrap()
            }
        }
    }

    fn refresh_food(&mut self) {
        self.food = Food {
            pos: Pos(
                random().mul(self.width as f64).floor() as i32,
                random().mul(self.height as f64).floor() as i32
            ),
            food_color: vec!["🟥", "🟧", "🟨", "🟩", "🟦", "🟪", "🟫"].get(random().mul(7.0).floor() as usize).unwrap()
        }
    }

    pub fn try_move(&mut self, direction: Direction) {
        let next = match direction {
            Direction::Down => Pos(0, 1),
            Direction::Up => Pos(0, -1),
            Direction::Left => Pos(-1, 0),
            Direction::Right => Pos(1, 0),
            _ => unreachable!(),
        };

        let head_next_position = self.snake.positions.front().unwrap() + &next;

        if self.snake.positions.get(1).unwrap() == &head_next_position {
            return;
        }

        if self.is_in_bounds(head_next_position) {
            self.snake.try_move(direction, &self.food);
            if head_next_position == self.food.pos {
                self.refresh_food();
            }
        } else {
            self.is_failed = true;
            self.snake.is_dead = true;
        }
    }

    fn is_in_bounds(&self, head_next_position: Pos) -> bool {
        head_next_position.0 >= 0
            && head_next_position.0 < self.width as i32
            && head_next_position.1 >= 0
            && head_next_position.1 < self.height as i32
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> + '_ {
        (0..self.height).flat_map(move |y| {
            (0..self.width).map(move |x| {
                // self.get_postion(&Pos(x as i32, y as i32))
                Pos(x as i32, y as i32)
            })
        })
    }

    pub fn get_position(&self, pos: &Pos) -> &str {
        if self.food.pos == *pos {
            return self.food.food_color;
        }
        if self
            .snake
            .positions
            .iter()
            .any(|p| p.0 == pos.0 && p.1 == pos.1)
        {
            return self.snake.color;
        }
        ""
    }
}
