use std::fmt::Display;

use super::{Board, Vec2};

pub struct BoardVec<T> {
    values: Vec<T>,
    size: Vec2,
}

impl<T> BoardVec<T> {
    fn calculate_index(&self, position: Vec2) -> usize {
        position.row * self.size.column + position.column
    }
}

impl<T: Copy + Display> Board for BoardVec<T> {
    type Item = T;

    fn new(size: Vec2, init_value: T) -> BoardVec<T> {
        let values = vec![init_value; size.row * size.column];

        BoardVec { values, size }
    }

    fn value(&self, position: Vec2) -> &Self::Item {
        &self.values[self.calculate_index(position)]
    }

    fn value_mut(&mut self, position: Vec2) -> &mut Self::Item {
        let index = self.calculate_index(position);
        &mut self.values[index]
    }

    fn size(&self) -> Vec2 {
        self.size
    }

    fn iter_row(&self, row: usize) -> impl Iterator<Item = &Self::Item> {
        self.values
            .iter()
            .skip(row * self.size.column)
            .take(self.size.column)
    }

    fn iter_row_mut(&mut self, row: usize) -> impl Iterator<Item = &mut Self::Item> {
        self.values
            .iter_mut()
            .skip(row * self.size.column)
            .take(self.size.column)
    }

    fn iter_column(&self, col: usize) -> impl Iterator<Item = &Self::Item> {
        self.values.iter().skip(col).step_by(self.size.column)
    }

    fn iter_column_mut(&mut self, col: usize) -> impl Iterator<Item = &mut Self::Item> {
        self.values.iter_mut().skip(col).step_by(self.size.column)
    }

    fn iter_all(&self) -> impl Iterator<Item = &Self::Item> {
        self.values.iter()
    }

    fn to_string(&self) -> String {
        let capacity = self.size.row * (self.size.column * 4 + 1);
        let mut result = String::with_capacity(capacity);
        for row in 0..self.size.row {
            for col in 0..self.size.column {
                use std::fmt::Write;
                let value = self.value(Vec2 { row, column: col });
                write!(result, "{} ", value).expect("writing to string should never fail");
            }
            result.push('\n');
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board_size = Vec2::new(5, 5);
        let board = BoardVec::new(board_size, 0);
        assert_eq!(board.size().row, 5);
        assert_eq!(board.size().column, 5);
    }

    #[test]
    fn test_board_get_set_value() {
        let board_size = Vec2::new(3, 3);
        let mut board = BoardVec::new(board_size, 0);
        let pos = Vec2::new(1, 1);
        *board.value_mut(pos) = 5;
        assert_eq!(*board.value(pos), 5);
    }

    #[test]
    fn test_board_get_row_line() {
        let board_size = Vec2::new(3, 3);
        let mut board = BoardVec::new(board_size, 0);
        *board.value_mut(Vec2::new(1, 0)) = 1;
        *board.value_mut(Vec2::new(1, 1)) = 2;
        *board.value_mut(Vec2::new(1, 2)) = 3;
        let row_line: Vec<usize> = board.iter_row(1).map(|&v| v).collect();
        println!("{}", board.to_string());
        assert_eq!(row_line, vec![1, 2, 3]);
    }
}
