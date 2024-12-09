use super::{cell::Cell, Solver};
use bit_set::BitSet;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum LineDirection {
    Row,
    Column,
}

impl LineDirection {
    pub fn opposite(&self) -> Self {
        match self {
            LineDirection::Row => LineDirection::Column,
            LineDirection::Column => LineDirection::Row,
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Line {
    pub packed: u32,
}

impl Line {
    pub fn new(direction: LineDirection, index: usize) -> Self {
        let dir_bit = match direction {
            LineDirection::Row => 0,
            LineDirection::Column => 1 << 31,
        };
        Self {
            packed: dir_bit | (index as u32),
        }
    }

    pub fn direction(&self) -> LineDirection {
        if (self.packed & (1 << 31)) == 0 {
            LineDirection::Row
        } else {
            LineDirection::Column
        }
    }

    pub fn index(&self) -> usize {
        (self.packed & !(1 << 31)) as usize
    }
}

#[derive(Debug)]
pub enum SolverError {
    InvalidSize(String),
    InvalidHint(String),
}

pub struct LineSolvingInfo<'a> {
    pub possibilities: &'a mut BitSet,
    pub given_hint: &'a Vec<usize>,
}

pub(super) trait LineSolvingInfoProvider {
    fn line_to_index(&self, line: Line) -> usize;
}

impl LineSolvingInfoProvider for Solver {
    fn line_to_index(&self, line: Line) -> usize {
        line.index()
            + if line.direction() == LineDirection::Row {
                0
            } else {
                self.board.size().row
            }
    }
}

pub(super) trait LineProcessor {
    fn get_line_cells(&self, line: Line) -> Vec<Cell>;
    fn update_line(&mut self, line: Line, new_cells: &[Cell]) -> Result<(), &'static str>;
}

impl LineProcessor for Solver {
    fn get_line_cells(&self, line: Line) -> Vec<Cell> {
        match line.direction() {
            LineDirection::Row => self.board.iter_row(line.index()).cloned().collect(),
            LineDirection::Column => self.board.iter_column(line.index()).cloned().collect(),
        }
    }

    fn update_line(&mut self, line: Line, new_cells: &[Cell]) -> Result<(), &'static str> {
        let iter_mut: Box<dyn Iterator<Item = &mut Cell>> = match line.direction() {
            LineDirection::Row => Box::new(self.board.iter_row_mut(line.index())),
            LineDirection::Column => Box::new(self.board.iter_column_mut(line.index())),
        };

        iter_mut
            .zip(new_cells.iter())
            .enumerate()
            .filter(|(_, (board_cell, &new_cell))| {
                new_cell != Cell::Crash && new_cell != Cell::Unknown && **board_cell != new_cell
            })
            .for_each(|(index, (board_cell, &new_cell))| {
                self.line_changed
                    .insert(Line::new(line.direction().opposite(), index));
                *board_cell = new_cell;
            });

        Ok(())
    }
}
