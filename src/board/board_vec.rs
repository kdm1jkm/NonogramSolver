use super::{Board, LineIterator, Vec2};

pub struct BoardVec<T> {
    values: Vec<T>,
    size: Vec2,
}

impl<T: Copy> Board<T> for BoardVec<T> {
    fn new(size: Vec2, init_value: T) -> BoardVec<T> {
        let values = vec![init_value; size.row * size.column];

        BoardVec { values, size }
    }

    fn get_value(&self, position: Vec2) -> T {
        let index = position.row * self.size.column + position.column;
        self.values[index]
    }

    fn get_row(&self) -> usize {
        self.size.row
    }

    fn get_column(&self) -> usize {
        self.size.column
    }

    fn any(&self, predicate: fn(T) -> bool) -> bool {
        self.values.iter().any(|&value| predicate(value))
    }

    fn get_row_line(&self, line: usize) -> impl Iterator<Item = T> + '_ {
        let start_position = Vec2 {
            row: line,
            column: 0,
        };
        let delta = Vec2 { row: 0, column: 1 };
        LineIterator::new(self, start_position, delta)
    }

    fn get_column_line(&self, line: usize) -> impl Iterator<Item = T> + '_ {
        let start_position = Vec2 {
            row: 0,
            column: line,
        };
        let delta = Vec2 { row: 1, column: 0 };
        LineIterator::new(self, start_position, delta)
    }

    fn set_value(&mut self, position: Vec2, value: T) {
        let index = position.row * self.size.column + position.column;
        self.values[index] = value;
    }
}
