pub mod board_formatter;
pub mod calculator;
pub mod cell;
pub mod line_processor;
pub mod solver_display;
pub mod solving_strategy;

use crate::board::{vec2::Vec2, Board};
use bit_set::BitSet;
use board_formatter::BoardFormatter;
use cell::Cell;
use line_processor::LineProcessor;
use solver_display::{SolverDisplay, SolverState, SolvingState};
use solving_strategy::SolvingStrategy;
use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
    sync::{Arc, RwLock},
};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum LineDirection {
    Row,
    Column,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Line {
    packed: u32,
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

pub struct Solver {
    board: Arc<RwLock<Board<Cell>>>,
    row_strategy: SolvingStrategy,
    column_strategy: SolvingStrategy,
    line_processor: LineProcessor,
    board_formatter: BoardFormatter,
    display: Option<Rc<RefCell<Box<dyn SolverDisplay>>>>,
    line_changed: HashSet<Line>,
}

impl Solver {
    fn validate_hints(size: usize, hints: &[Vec<usize>], is_row: bool) -> Result<(), SolverError> {
        let dimension = if is_row { "row" } else { "column" };

        for (idx, hint) in hints.iter().enumerate() {
            let sum: usize = hint.iter().sum();
            let spaces_needed = sum + hint.len() - 1;
            if spaces_needed > size {
                return Err(SolverError::InvalidHint(format!(
                    "{} {} hint sum {} (with spaces) exceeds size {}",
                    dimension, idx, spaces_needed, size
                )));
            }
        }

        Ok(())
    }

    pub fn new(
        size: Vec2,
        row_hint: Vec<Vec<usize>>,
        column_hint: Vec<Vec<usize>>,
        display: Box<dyn SolverDisplay>,
    ) -> Result<Self, SolverError> {
        let display = Rc::new(RefCell::new(display));
        display
            .borrow_mut()
            .change_state(SolverState::Loading("Validate board size...".to_string()));

        if size.row == 0 || size.column == 0 {
            return Err(SolverError::InvalidSize(format!(
                "Invalid board size: {}x{}",
                size.row, size.column
            )));
        }

        display
            .borrow_mut()
            .change_state(SolverState::Loading("Validate hints...".to_string()));

        Self::validate_hints(size.column, &row_hint, true)?;
        Self::validate_hints(size.row, &column_hint, false)?;

        let board = Arc::new(RwLock::new(Board::new(size, Cell::Unknown)));

        let mut solver = Self {
            board: Arc::clone(&board),
            row_strategy: SolvingStrategy::new(
                size.column,
                row_hint.clone(),
                Box::new(DisplayRef::new(Rc::clone(&display))),
            ),
            column_strategy: SolvingStrategy::new(
                size.row,
                column_hint.clone(),
                Box::new(DisplayRef::new(Rc::clone(&display))),
            ),
            line_processor: LineProcessor::new(Arc::clone(&board)),
            board_formatter: BoardFormatter::new(board, row_hint, column_hint),
            display: Some(display),
            line_changed: HashSet::new(),
        };

        if let Some(display) = &solver.display {
            display
                .borrow_mut()
                .change_state(SolverState::Loading("Initialize solver...".to_string()));
        }

        for r in 0..size.row {
            solver.line_changed.insert(Line::new(LineDirection::Row, r));
        }
        for c in 0..size.column {
            solver
                .line_changed
                .insert(Line::new(LineDirection::Column, c));
        }

        if let Some(display) = &solver.display {
            display.borrow_mut().change_state(SolverState::Idle);
        }

        Ok(solver)
    }

    pub fn to_string(&self) -> String {
        self.board_formatter.to_string()
    }

    pub fn is_solved(&self) -> bool {
        self.board
            .read()
            .unwrap()
            .iter_all()
            .all(|cell| *cell != Cell::Unknown)
    }

    fn solve_line(&mut self, line: Line) -> Result<(), SolverError> {
        let line_cells = self.line_processor.get_line_cells(line);
        let line_length = line_cells.len();

        let new_cells = (match line.direction() {
            LineDirection::Row => {
                self.row_strategy
                    .solve_line(line.index(), &line_cells, line_length)
            }
            LineDirection::Column => {
                self.column_strategy
                    .solve_line(line.index(), &line_cells, line_length)
            }
        })
        .map_err(|e| SolverError::InvalidHint(e.to_string()))?;

        let mut changed_lines = BitSet::new();
        self.line_processor
            .update_line(line, &new_cells, &mut changed_lines)
            .map_err(|e| SolverError::InvalidHint(e.to_string()))?;

        for changed_index in changed_lines.iter() {
            self.line_changed.insert(Line::new(
                match line.direction() {
                    LineDirection::Row => LineDirection::Column,
                    LineDirection::Column => LineDirection::Row,
                },
                changed_index,
            ));
        }

        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), SolverError> {
        while let Some(line) = self.next_line_pop() {
            if let Some(display) = &self.display {
                display
                    .borrow_mut()
                    .change_state(SolverState::Solving(SolvingState {
                        board: Arc::clone(&self.board),
                        line,
                        line_waiting: self.line_order(),
                    }));
            }
            self.solve_line(line)?;
        }

        if let Some(display) = &self.display {
            display.borrow_mut().change_state(SolverState::Idle);
        }
        Ok(())
    }

    fn get_line_sort_key(&self, line: &Line) -> usize {
        match line.direction() {
            LineDirection::Row => self.row_strategy.get_line_possibility_count(line.index()),
            LineDirection::Column => self
                .column_strategy
                .get_line_possibility_count(line.index()),
        }
    }

    fn next_line(&self) -> Option<Line> {
        self.line_changed
            .iter()
            .min_by_key(|line| self.get_line_sort_key(line))
            .cloned()
    }

    fn next_line_pop(&mut self) -> Option<Line> {
        let line = self.next_line()?;
        self.line_changed.remove(&line);
        Some(line)
    }

    pub fn line_order(&self) -> Vec<Line> {
        let mut order = self.line_changed.iter().cloned().collect::<Vec<_>>();
        order.sort_by_key(|line| self.get_line_sort_key(line));
        order
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        drop(self.display.take());
    }
}

struct DisplayRef {
    display: Rc<RefCell<Box<dyn SolverDisplay>>>,
}

impl DisplayRef {
    fn new(display: Rc<RefCell<Box<dyn SolverDisplay>>>) -> Self {
        Self { display }
    }
}

impl SolverDisplay for DisplayRef {
    fn change_state(&mut self, state: SolverState) {
        self.display.borrow_mut().change_state(state);
    }

    fn update_progress(&mut self, progress: (usize, usize)) {
        self.display.borrow_mut().update_progress(progress);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::console::ConsoleSolverDisplay;
    use Cell::*;

    #[test]
    fn test_solve_line() {
        let mut solver: Solver = Solver::new(
            Vec2 {
                row: 10,
                column: 10,
            },
            vec![
                vec![2, 2, 2],
                vec![2, 3, 2],
                vec![2, 3, 3],
                vec![2, 2, 3],
                vec![2, 2, 2],
                vec![3, 2, 2],
                vec![3, 3, 2],
                vec![2, 3, 2],
                vec![2, 2, 2],
                vec![2, 2, 2],
            ],
            vec![
                vec![10],
                vec![10],
                vec![2],
                vec![2],
                vec![10],
                vec![10],
                vec![2],
                vec![2],
                vec![10],
                vec![10],
            ],
            Box::new(ConsoleSolverDisplay::new_with_default()),
        )
        .unwrap();

        solver.solve().unwrap();

        let expected_board = vec![
            vec![
                Block, Block, Blank, Blank, Block, Block, Blank, Blank, Block, Block,
            ],
            vec![
                Block, Block, Blank, Block, Block, Block, Blank, Blank, Block, Block,
            ],
            vec![
                Block, Block, Blank, Block, Block, Block, Blank, Block, Block, Block,
            ],
            vec![
                Block, Block, Blank, Blank, Block, Block, Blank, Block, Block, Block,
            ],
            vec![
                Block, Block, Blank, Blank, Block, Block, Blank, Blank, Block, Block,
            ],
            vec![
                Block, Block, Block, Blank, Block, Block, Blank, Blank, Block, Block,
            ],
            vec![
                Block, Block, Block, Blank, Block, Block, Block, Blank, Block, Block,
            ],
            vec![
                Block, Block, Blank, Blank, Block, Block, Block, Blank, Block, Block,
            ],
            vec![
                Block, Block, Blank, Blank, Block, Block, Blank, Blank, Block, Block,
            ],
            vec![
                Block, Block, Blank, Blank, Block, Block, Blank, Blank, Block, Block,
            ],
        ];

        for row in 0..solver.board.read().unwrap().size().row {
            for col in 0..solver.board.read().unwrap().size().column {
                assert_eq!(
                    solver
                        .board
                        .read()
                        .unwrap()
                        .value(Vec2 { row, column: col }),
                    &expected_board[row][col]
                );
            }
        }
    }
}
