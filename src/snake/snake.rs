use std::{collections::VecDeque, ops::Add, mem::swap};
use super::{pos::Pos, gameboard::Direction, food::Food};




#[derive(Debug, Clone)]
pub struct Snake<'a> {
    pub positions: VecDeque<Pos>,
    pub is_dead: bool,
    pub color: &'a str
}

impl Add<Pos> for Snake<'_> {
    type Output = Self;
    fn add(self, rhs: Pos) -> Self::Output {
        let next_head = self.positions.front().unwrap() + &rhs;
        let mut positions = self.positions.clone();
        positions.push_front(next_head);
        Self {
            positions,
            color: self.color,
            is_dead: self.is_dead
        }
    }
}


impl Snake<'_> {
    pub fn new(positions: Vec<Pos>) -> Self {
        Self {
            positions: VecDeque::from(positions),
            is_dead: false,
            color: "🟥"
        }
    }

    pub fn try_move(&mut self, direction: Direction, food: &Food) {
        let next =  match direction {
            Direction::Down => Pos(0, 1),
            Direction::Up => Pos(0, -1),
            Direction::Left => Pos(-1, 0),
            Direction::Right => Pos(1, 0),
            _ => unreachable!()
        };

        let head_next_position = self.positions.front().unwrap() + &next;

        if self.is_eat_self(&head_next_position) {
            self.is_dead = true;
            return;
        }
        let mut new_snake = self.clone() + next;
        if head_next_position != food.pos {
            new_snake.positions.pop_back();
        } 
        swap(self, &mut new_snake);
        
    }

    pub fn is_eat_self(&self, head_next_position: &Pos) -> bool {
        self.positions.contains(head_next_position)
    }

}
