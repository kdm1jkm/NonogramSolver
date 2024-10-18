#[derive(Copy, Clone)]
pub struct Vec2 {
    pub row: usize,
    pub column: usize,
}

impl Vec2{
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}