pub mod calculator;
pub mod cell;
pub mod error;
pub mod parser;
pub mod solver_display;
pub mod utils;

use crate::board::{vec2::Vec2, Board};
use bit_set::BitSet;
use calculator::number_distribution_calculator::NumberDistributionCalculator;
use cell::Cell;
use error::{InvalidInfoError, SolverError, SolvingError};
use rand::{seq::SliceRandom, thread_rng};
use solver_display::{SolverDisplay, SolverState, SolvingState};
use std::collections::HashSet;
use utils::{Line, LineDirection, LineProcessor, LineSolvingInfoProvider};

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
        display.change_state(SolverState::Loading("Validate board size...".to_string()));

        if size.row == 0 || size.column == 0 {
            return Err(SolverError::InvalidInitialInfo(InvalidInfoError {
                error_line: Line::new(LineDirection::Row, 0),
                size: 0,
                message: "Invalid board size: size must be greater than 0".to_string(),
            }));
        }

        display.change_state(SolverState::Loading("Validate hints...".to_string()));

        Self::validate_hints(size.column, &row_hint, LineDirection::Row)?;
        Self::validate_hints(size.row, &column_hint, LineDirection::Column)?;

        let board = Board::new(size, Cell::Unknown);

        let mut calculator = NumberDistributionCalculator::new();

        let given_hint = row_hint
            .into_iter()
            .chain(column_hint.into_iter())
            .collect::<Vec<_>>();

        let possibilities = given_hint
            .iter()
            .map(|hint| {
                let count = calculator.calc_distribute_count_line_hint(hint, size.column);
                let mut possibility = BitSet::with_capacity(count);
                for j in 0..count {
                    possibility.insert(j);
                }
                possibility
            })
            .collect::<Vec<_>>();

        let possibility_count = possibilities.iter().map(BitSet::len).collect();

        let mut solver = Self {
            line_changed: HashSet::new(),
            display,
            calculator,
            board,
            possibilities,
            possibility_count,
            given_hint,
        };

        solver
            .display
            .change_state(SolverState::Loading("Initialize solver...".to_string()));

        for r in 0..size.row {
            solver
                .line_changed
                .insert(Line::new(utils::LineDirection::Row, r));
        }
        for c in 0..size.column {
            solver
                .line_changed
                .insert(Line::new(utils::LineDirection::Column, c));
        }

        solver.display.change_state(SolverState::Idle);

        Ok(solver)
    }

    pub fn is_solved(&self) -> bool {
        self.board.iter_all().all(|cell| *cell != Cell::Unknown)
    }

    fn solve_line(&mut self, line: Line) -> Result<(), SolverError> {
        let line_cells = self.get_line_cells(line);
        let line_length = line_cells.len();

        if !line_cells.contains(&Cell::Unknown) {
            return Ok(());
        }

        let line_index = self.line_to_index(line);
        let mut possibilities = self.possibilities[line_index].iter().collect::<Vec<_>>();
        let hint = &self.given_hint[line_index];

        let mut new_line = vec![Cell::Unknown; line_length];
        let mut indexed_line = Vec::new();

        let mut remove_possibility = BitSet::with_capacity(possibilities.len());

        possibilities.shuffle(&mut thread_rng());

        let total_possibilities = possibilities.len();

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
                    SolverError::SolvingError(SolvingError {
                        current_line: line_cells.clone(),
                        calculating_line: indexed_line.clone(),
                        hint: hint.clone(),
                        error_line: line,
                        message: e,
                    })
                })?;

            if indexed_line
                .iter()
                .zip(line_cells.iter())
                .any(|(indexed_cell, cell)| (*indexed_cell | *cell) == Cell::Crash)
            {
                remove_possibility.insert(possibility_index);
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
                .zip(line_cells.iter())
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
                .change_state(SolverState::Solving(SolvingState {
                    board: &self.board,
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

    pub fn line_order(&self) -> Vec<Line> {
        let mut order = self.line_changed.iter().cloned().collect::<Vec<_>>();
        order.sort_by_key(|line| self.get_line_sort_key(*line));
        order
    }

    pub fn get_line_possibility_count(&self, line_index: usize) -> usize {
        self.possibility_count[line_index]
    }

    pub fn update_possibility_count(&mut self, line_index: usize) {
        self.possibility_count[line_index] = self.possibilities[line_index].len();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::console::ConsoleSolverDisplay;
    use crate::solver::parser::{FileSolverParser, SolverParser};

    #[test]
    fn test_solve() {
        let mut solver: Solver = FileSolverParser::new("./sample/data1.txt")
            .create_solver(Box::new(ConsoleSolverDisplay::new_with_default()))
            .unwrap();

        solver.solve().unwrap();

        assert!(solver.is_solved());
    }
}
