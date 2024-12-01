mod board_vec;
mod vec2;

use std::fmt::Display;

pub use board_vec::BoardVec;
pub use vec2::Vec2;

pub trait Board {
    type Item: Copy + Display;

    fn new(size: Vec2, init_value: Self::Item) -> Self;
    fn value(&self, position: Vec2) -> &Self::Item;
    fn value_mut(&mut self, position: Vec2) -> &mut Self::Item;
    fn size(&self) -> Vec2;

    fn iter_all(&self) -> impl Iterator<Item = &Self::Item>;

    fn iter_row(&self, row: usize) -> impl Iterator<Item = &Self::Item>;
    fn iter_row_mut(&mut self, row: usize) -> impl Iterator<Item = &mut Self::Item>;
    fn iter_column(&self, col: usize) -> impl Iterator<Item = &Self::Item>;
    fn iter_column_mut(&mut self, col: usize) -> impl Iterator<Item = &mut Self::Item>;

    fn to_string(&self) -> String;
}
