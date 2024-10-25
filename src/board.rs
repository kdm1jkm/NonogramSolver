mod board_vec;
mod vec2;

pub use board_vec::BoardVec;
pub use vec2::Vec2;

pub trait Board {
    type Item: Copy;

    fn new(size: Vec2, init_value: Self::Item) -> Self;
    fn value(&self, position: Vec2) -> &Self::Item;
    fn value_mut(&mut self, position: Vec2) -> &mut Self::Item;
    fn size(&self) -> Vec2;

    fn row(&self, row: usize) -> impl Iterator<Item = &Self::Item>;
    fn row_mut(&mut self, row: usize) -> impl Iterator<Item = &mut Self::Item>;
    fn column(&self, col: usize) -> impl Iterator<Item = &Self::Item>;
    fn column_mut(&mut self, col: usize) -> impl Iterator<Item = &mut Self::Item>;
}
