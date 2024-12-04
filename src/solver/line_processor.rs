use super::{cell::Cell, Line, LineDirection};
use crate::board::Board;
use bit_set::BitSet;
use std::sync::{Arc, RwLock};

pub struct LineProcessor {
    board: Arc<RwLock<Board<Cell>>>,
}

impl LineProcessor {
    pub fn new(board: Arc<RwLock<Board<Cell>>>) -> Self {
        Self { board }
    }

    pub fn get_line_cells(&self, line: Line) -> Vec<Cell> {
        let board = self.board.read().unwrap();
        match line.direction() {
            LineDirection::Row => board.iter_row(line.index()).cloned().collect(),
            LineDirection::Column => board.iter_column(line.index()).cloned().collect(),
        }
    }

    pub fn update_line(
        &mut self,
        line: Line,
        new_cells: &[Cell],
        changed_lines: &mut BitSet,
    ) -> Result<(), &'static str> {
        let mut board = self.board.write().unwrap();
        let iter_mut: Box<dyn Iterator<Item = &mut Cell>> = match line.direction() {
            LineDirection::Row => Box::new(board.iter_row_mut(line.index())),
            LineDirection::Column => Box::new(board.iter_column_mut(line.index())),
        };

        iter_mut
            .zip(new_cells.iter())
            .enumerate()
            .filter(|(_, (_, &new_cell))| new_cell != Cell::Crash)
            .filter(|(_, (_, &new_cell))| new_cell != Cell::Unknown)
            .filter(|(_, (board_cell, &new_cell))| **board_cell != new_cell)
            .for_each(|(index, (board_cell, &new_cell))| {
                changed_lines.insert(match line.direction() {
                    LineDirection::Row => index,
                    LineDirection::Column => index,
                });
                *board_cell = new_cell;
            });

        Ok(())
    }
}
