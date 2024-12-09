pub mod calculator;
mod cell;
pub mod error;
pub mod parser;
pub mod solver_display;
pub mod types;

pub use cell::Cell;

use crate::board::{Board, Vec2};
use bit_set::BitSet;
use calculator::NumberDistributionCalculator;
use error::{InvalidInfoError, SolverError, SolvingError};
use solver_display::{SolverDisplay, SolverState, SolvingContext};
use std::collections::HashSet;
use types::{Line, LineDirection, LineProcessor, LineSolvingInfoProvider};
pub struct Solver {
    // Fixed
    given_hint: Vec<Vec<usize>>,
    display: Box<dyn SolverDisplay>,

    // Mutable
    pub board: Board<Cell>,
    possibilities: Vec<BitSet>,

    // Cache
    possibility_count: Vec<usize>,
    line_changed: HashSet<Line>,
    calculator: NumberDistributionCalculator,
}

impl Solver {
    fn validate_hints(
        size: usize,
        hints: &[Vec<usize>],
        direction: LineDirection,
    ) -> Result<(), SolverError> {
        for (idx, hint) in hints.iter().enumerate() {
            if hint.is_empty() {
                continue;
            }
            let sum: usize = hint.iter().sum();
            let spaces_needed = sum + hint.len() - 1;
            if spaces_needed > size {
                return Err(SolverError::InvalidInitialInfo(InvalidInfoError {
                    error_line: Line::new(direction, idx),
                    size,
                    message: "Invalid hint: required space for hint is larger than the board size"
                        .to_string(),
                }));
            }
        }

        Ok(())
    }

    pub fn new(
        size: Vec2,
        row_hint: Vec<Vec<usize>>,
        column_hint: Vec<Vec<usize>>,
        mut display: Box<dyn SolverDisplay>,
    ) -> Result<Self, SolverError> {
        display.change_state(SolverState::Loading("Validating board size.".to_string()));

        if size.row == 0 || size.column == 0 {
            return Err(SolverError::InvalidBoardSize(size.column, size.row));
        }

        display.change_state(SolverState::Loading("Validating hints.".to_string()));

        Self::validate_hints(size.column, &row_hint, LineDirection::Row)?;
        Self::validate_hints(size.row, &column_hint, LineDirection::Column)?;

        let board = Board::new(size, Cell::Unknown);

        let mut calculator = NumberDistributionCalculator::new();

        display.change_state(SolverState::Loading(
            "Calculating initial possibilities.".to_string(),
        ));
        let possibilities = [(&row_hint, size.column), (&column_hint, size.row)]
            .into_iter()
            .flat_map(|(hints, size)| hints.iter().map(move |hint| (hint, size)))
            .map(|(hint, size)| calculator.calc_distribute_count_line_hint(hint, size))
            .map(|count| (0..count).collect::<BitSet>())
            .collect::<Vec<_>>();
        let given_hint = row_hint.into_iter().chain(column_hint).collect::<Vec<_>>();

        let possibility_count = possibilities.iter().map(BitSet::len).collect();

        let line_changed = [
            (0..size.row, LineDirection::Row),
            (0..size.column, LineDirection::Column),
        ]
        .into_iter()
        .flat_map(|(range, direction)| range.map(move |index| Line::new(direction, index)))
        .collect::<HashSet<_>>();

        display.change_state(SolverState::Idle);

        Ok(Self {
            line_changed,
            display,
            calculator,
            board,
            possibilities,
            possibility_count,
            given_hint,
        })
    }

    pub fn is_solved(&self) -> bool {
        self.board.iter_all().all(|cell| *cell != Cell::Unknown)
    }

    fn solve_line(&mut self, line: Line) -> Result<(), SolverError> {
        let current_line = self.get_line_cells(line);
        if current_line.iter().all(|&cell| cell != Cell::Unknown) {
            return Ok(());
        }

        let line_length = current_line.len();
        let mapped_line_index = self.line_to_index(line);
        let hint = &self.given_hint[mapped_line_index];
        let possibilities = self.possibilities[mapped_line_index]
            .iter()
            .collect::<Vec<_>>();
        let total_possibilities = possibilities.len();

        let mut new_line = vec![Cell::Unknown; line_length];
        let mut indexed_line = Vec::new();

        if hint.is_empty() {
            let new_line = vec![Cell::Blank; line_length];
            self.update_line(line, &new_line);
            return Ok(());
        }

        for (i, possibility_index) in possibilities.into_iter().enumerate() {
            self.display.update_progress((i + 1, total_possibilities));

            self.calculator
                .calc_distribute_number_line_hint(
                    hint,
                    line_length,
                    possibility_index,
                    &mut indexed_line,
                )
                .map_err(|e| {
                    SolverError::InvalidSolvingState(SolvingError {
                        current_line: current_line.clone(),
                        calculating_line: indexed_line.clone(),
                        hint: hint.clone(),
                        error_line: line,
                        message: e,
                    })
                })?;

            if indexed_line
                .iter()
                .zip(current_line.iter())
                .any(|(indexed_cell, cell)| (*indexed_cell | *cell) == Cell::Crash)
            {
                self.possibilities[mapped_line_index].remove(possibility_index);
                continue;
            }

            new_line
                .iter_mut()
                .zip(indexed_line.iter())
                .for_each(|(cell, &indexed_cell)| {
                    *cell = *cell | indexed_cell;
                });

            if new_line
                .iter()
                .zip(current_line.iter())
                .all(|(new_cell, line_cell)| {
                    *line_cell != Cell::Unknown && *new_cell == Cell::Crash
                })
            {
                break;
            }
        }

        self.update_line(line, &new_line);

        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), SolverError> {
        while let Some(line) = self.next_line_pop() {
            self.display
                .change_state(SolverState::Solving(SolvingContext {
                    board: self.board.clone(),
                    line,
                    line_waiting: self.line_order(),
                }));
            self.solve_line(line)?;
        }

        self.display.change_state(SolverState::Solved);
        Ok(())
    }

    fn get_line_sort_key(&self, line: Line) -> usize {
        self.possibility_count[self.line_to_index(line)]
    }

    fn next_line(&self) -> Option<Line> {
        self.line_changed
            .iter()
            .min_by_key(|line| self.get_line_sort_key(**line))
            .cloned()
    }

    fn next_line_pop(&mut self) -> Option<Line> {
        let line = self.next_line()?;
        self.line_changed.remove(&line);
        Some(line)
    }

    fn line_order(&self) -> Vec<Line> {
        let mut order = self.line_changed.iter().cloned().collect::<Vec<_>>();
        order.sort_by_key(|line| self.get_line_sort_key(*line));
        order
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        display::ConsoleDisplay,
        solver::parser::{FileSolverParser, SolverParser},
    };

    #[test]
    fn test_solve() {
        let mut solver: Solver = FileSolverParser::new("./sample/data1.txt")
            .create_solver(Box::new(ConsoleDisplay::new_with_default()))
            .unwrap();

        solver.solve().unwrap();

        assert!(solver.is_solved());
    }
}
