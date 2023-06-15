


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos(pub i8, pub i8);

impl Pos {
    pub fn key(&self) -> String {
        let mut res = self.0.to_string();
        res.push_str(self.1.to_string().as_str());
        res
    }
}

