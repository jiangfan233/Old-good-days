use rand::random;
use crate::pos::Pos;
use std::{collections::HashSet, ops::Add};



#[derive(Debug, Clone)]
pub struct Shape {
    pub positions: HashSet<Pos>,
    pub anchor: Pos,
    pub typ: &'static str,
}


macro_rules! shape_factory {
    ( $( $fn:ident - $typ:literal - $anchor:expr => $( $positions:expr),* ;)* )=> {

        $(
            pub fn $fn() -> Self {
                Self {
                    typ: $typ,
                    positions: ( $($positions),* ).into_iter().collect::<HashSet<Pos>>(),
                    anchor: $anchor,
                }
            }
        )*

    };
}


impl Add<Pos> for &Shape {
    type Output = Shape;

    fn add(self, rhs: Pos) -> Self::Output {
        let Pos(a, b) = self.anchor;
        Shape {
            anchor: Pos(a + rhs.0, b + rhs.1),
            typ: self.typ,
            positions: self
                .positions
                .iter()
                .map(|pos| Pos(pos.0 + rhs.0, pos.1 + rhs.1))
                .collect(),
        }
    }
}


impl Shape {
    pub fn new(positions: HashSet<Pos>, anchor: Pos, typ: &'static str) -> Self {
        Self {
            positions,
            anchor,
            typ,
        }
    }

    shape_factory! {
        new_i - "🟦" - Pos(1, 0) => [Pos(0,0), Pos(1,0), Pos(2,0), Pos(3,0), ];
        new_o - "🟨" - Pos(1, 0) => [Pos(0,0), Pos(1,0), Pos(0,1), Pos(1,1), ];
        new_t - "🟪" - Pos(1, 0) => [Pos(0,0), Pos(1,0), Pos(2,0), Pos(1,1), ];
        new_j - "⬛" - Pos(1, 1) => [Pos(0,2), Pos(1,0), Pos(1,1), Pos(1,2), ];
        new_l - "🟧" - Pos(0, 1) => [Pos(0,0), Pos(0,1), Pos(0,2), Pos(1,2), ];
        new_s - "🟩" - Pos(1, 0) => [Pos(1,0), Pos(2,0), Pos(0,1), Pos(1,1), ];
        new_z - "🟥" - Pos(1, 0) => [Pos(0,0), Pos(1,0), Pos(1,1), Pos(2,1), ];
    }

    pub fn random_shape() -> Self {
        match (random::<f64>() * 7.0).floor() as u8 {
            0 => Self::new_i(),
            1 => Self::new_o(),
            2 => Self::new_t(),
            3 => Self::new_j(),
            4 => Self::new_l(),
            5 => Self::new_s(),
            6 => Self::new_z(),
            _ => unreachable!(),
        }
    }

    pub fn rotate(&self) -> Self {
        let Pos(a, b) = self.anchor;
        Self {
            positions: self
                .positions
                .iter()
                .map(|Pos(x, y)| Pos(-y + a + b, b + x - a))
                .collect(),
            anchor: self.anchor,
            typ: self.typ,
        }
    }

    pub fn remove(&mut self, y:i8) {
        self.positions = self.positions
            .iter()
            .copied()
            .filter(|pos| pos.1 != y)
            .map(|pos| {
                if pos.1 < y {
                    Pos(pos.0, pos.1 + 1)
                } else {
                    pos
                }
            })
            .collect();
            
    }

}

mod tests {
    use crate::shape::Shape;

    #[test]
    fn test() {
        println!("{:#?}", Shape::new_o());
    }
}
