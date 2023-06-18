use std::ops::Add;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pos (pub i32, pub i32);

impl Add for &Pos {
    type Output = Pos;

    fn add(self, rhs: &Pos) -> Self::Output {
        Pos( self.0 + rhs.0, self.1 + rhs.1 )
    }
}

impl Pos {
    pub fn key(&self) -> String {
        let mut res = self.0.to_string();
        res.push_str(self.1.to_string().as_str());
        res
    }
}
