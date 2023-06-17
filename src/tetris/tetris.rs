
use crate::TetrisLib::{pos::Pos, shape::Shape};
use crate::StateLib::persist::{PersistedOrigin, Persist};

use std::mem::replace;

pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Tetris {
    pub current_shape: Shape,
    pub fixed_shapes: Vec<Shape>,
    pub width: u8,
    pub height: u8,
    pub failed: bool,
}

impl Persist for Tetris {
    fn ptr(&self) -> crate::StateLib::persist::PersistedOrigin {
        PersistedOrigin
    }
}


impl Default for Tetris {
    fn default() -> Self {
        Self {
            current_shape: &Shape::random_shape() + Pos((10 / 2 - 1) as i8, 0),
            fixed_shapes: vec![],
            width: 10,
            height: 25,
            failed: false,
        }
    }
}




impl Tetris {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            current_shape: &Shape::random_shape() + Pos((width / 2 - 1) as i8, 0),
            fixed_shapes: vec![],
            failed: false,
            width,

            height,
        }
    }

    // move the current shape down by one row.
    pub fn tick(&mut self) -> Option<Shape> {
        let new_shape = &self.current_shape + Pos(0, 1);


        if !self.could_move(&new_shape) {
            // need to change current shape.
            // and put the old shape into fixed_shapes
            let fixed_shape = replace(
                &mut self.current_shape,
                &Shape::random_shape() + Pos((self.width / 2 - 1) as i8, 0),
            );
            self.fixed_shapes.push(fixed_shape);

            self.remove_full_lines();

            if self.is_colliding_with(&self.current_shape) {
                // Game over
                self.failed = true;
            }
            None
        } else {
            // self.current_shape = new_shape;
            let prev_shape = replace(&mut self.current_shape, new_shape);
            Some(prev_shape)
        }
    }

    pub fn is_out_of_bounds(&self, shape: &Shape) -> bool {
        !shape.positions.iter().all(|pos| {
            pos.0 >= 0 && pos.0 < self.width as i8 && pos.1 >= 0 && pos.1 < self.height as i8
        })
    }

    pub fn is_colliding_with(&self, shape: &Shape) -> bool {
        self.fixed_shapes
            .iter()
            .any(|fixed_shape| fixed_shape.positions.intersection(&shape.positions).count() > 0)
    }

    pub fn could_move(&self, shape: &Shape) -> bool {
        !self.is_out_of_bounds(shape) && !self.is_colliding_with(shape)
    }

    pub fn shift(&mut self, direction: Direction) -> Option<Shape> {
        let new_shape = &self.current_shape
            + match direction {
                Direction::Left => Pos(-1, 0),
                Direction::Right => Pos(1, 0),
            };
        if self.could_move(&new_shape) {
            let prev = replace(&mut self.current_shape, new_shape);
            Some(prev)
        } else {
            None
        }
        
    }

    pub fn rotate(&mut self) -> Option<Shape> {
        let new_shape = self.current_shape.rotate();
        if self.could_move(&new_shape) {
            let prev = replace(&mut self.current_shape, new_shape);
            Some(prev)
        } else {
            None
        }
    }

    pub fn is_line_full(&self, y: i8) -> bool {
        self.fixed_shapes
            .iter()
            .flat_map(|shape| shape.positions.iter().filter(|pos| pos.1 == y))
            .filter(|pos| pos.1 == y)
            .count()
            == self.width as usize
    }

    pub fn remove_full_lines(&mut self) {
        for y in 0..self.height {
            if self.is_line_full(y as i8) {
                self.fixed_shapes
                    .iter_mut()
                    .for_each(|shape: &mut Shape| shape.remove(y as i8))
            }
        }
    }

    pub fn getPosition(&self, pos: Pos) -> &str {
        let res = self
            .current_shape
            .positions
            .iter()
            .find(|p| pos.0 == p.0 && pos.1 == p.1);
        if res.is_some() {
            return self.current_shape.typ;
        }

        let res = self
            .fixed_shapes
            .iter()
            .find(|shape| shape.positions.iter().any(|p| p.0 == pos.0 && p.1 == pos.1));
        if let Some(shape) = res {
            return shape.typ;
        }

        ""
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> {
        let width = self.width;
        let height = self.height;

        (0..height).flat_map(move |y| (0..width).map(move |x| Pos(x as i8, y as i8)))
    }

}

mod test {
    use crate::TetrisLib::tetris::Tetris;

    #[test]
    fn testNewTetris() {
        println!("{:#?}", Tetris::new(10, 25));
    }

    #[test]
    fn testDefaultTetris() {
        println!("{:#?}", Tetris::default());
    }

    #[test]
    fn testTick() {
        let mut tetris = Tetris::default();

        for _ in 0..tetris.height {
            tetris.tick();
        }

        assert_eq!(tetris.fixed_shapes.len(), 1);
        // println!("{:#?}", tetris);
    }
}
