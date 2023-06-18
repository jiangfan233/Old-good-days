use super::pos::Pos;



pub struct  Food<'a> {
    pub food_color: &'a str,
    pub pos: Pos
}