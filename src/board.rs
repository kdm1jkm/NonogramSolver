mod board_vec;
mod vec2;

use board_vec::BoardVec;
use std::marker::PhantomData;
pub use vec2::Vec2;

pub trait Board<T> {
    fn new(size: Vec2, init_value: T) -> Self;
    fn get_value(&self, position: Vec2) -> T;
    fn get_row(&self) -> usize;
    fn get_column(&self) -> usize;
    fn any(&self, predicate: fn(T) -> bool) -> bool;
    fn get_row_line(&self, row: usize) -> impl Iterator<Item = T> + '_;
    fn get_column_line(&self, column: usize) -> impl Iterator<Item = T> + '_;
    fn set_value(&mut self, position: Vec2, value: T);
}

impl<'a, T, U: Board<T>> Iterator for LineIterator<'a, T, U> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let current_position = Vec2 {
            row: self.start_position.row + self.delta.row * self.index,
            column: self.start_position.column + self.delta.column * self.index,
        };

        if current_position.row >= self.board.get_row()
            || current_position.column >= self.board.get_column()
        {
            return None;
        }

        self.index += 1;
        Some(self.board.get_value(current_position))
    }
}

impl<'a, T, U: Board<T>> LineIterator<'a, T, U> {
    pub fn new(board: &'a U, start_position: Vec2, delta: Vec2) -> LineIterator<'a, T, U> {
        LineIterator {
            board,
            start_position,
            delta,
            index: 0,
            _marker: PhantomData,
        }
    }
}

pub struct LineIterator<'a, T, U: Board<T>> {
    board: &'a U,
    start_position: Vec2,
    delta: Vec2,
    index: usize,
    _marker: PhantomData<T>,
}

pub fn default_board<T: Copy>(size: Vec2, init_value: T) -> impl Board<T> {
    BoardVec::new(size, init_value)
}
