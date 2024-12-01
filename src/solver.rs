mod cell;
use bit_set::BitSet;
use calculator::DistributeNumberCalculator;
use cell::Cell;

use crate::board::{Board, BoardVec, Vec2};
pub mod calculator;

impl DistributeNumberCalculator {
    fn calc_distribute_number_line_hint(
        &mut self,
        hint_numbers: &Vec<usize>,
        length: usize,
        index: usize,
    ) -> Result<Vec<Cell>, &'static str> {
        let mut result = Vec::with_capacity(length);

        let distribute = self.calc_distribute_number(
            length + 1 - hint_numbers.iter().sum::<usize>() - hint_numbers.len(),
            hint_numbers.len() + 1,
            index,
        )?;

        for i in 0..hint_numbers.len() {
            result.append(&mut vec![Cell::Blank; distribute[i]]);
            result.append(&mut vec![Cell::Block; hint_numbers[i]]);
            if i < hint_numbers.len() - 1 {
                result.push(Cell::Blank);
            }
        }

        result.append(&mut vec![Cell::Blank; distribute[distribute.len() - 1]]);

        return Ok(result);
    }

    fn calc_distribute_count_line_hint(
        &mut self,
        hint_numbers: &Vec<usize>,
        length: usize,
    ) -> usize {
        self.comb_counter.calc_comb_count(
            length + 1 - hint_numbers.iter().sum::<usize>() - hint_numbers.len(),
            hint_numbers.len() + 1,
        )
    }
}

struct SolvingInfo<T: Board<Item = Cell>> {
    count_board: T,
    possibility_count: Vec<usize>,
    possibilities: Vec<BitSet>,
    given_hint: Vec<Vec<usize>>,
}

enum LineDirection {
    Row,
    Column,
}

struct Line {
    direction: LineDirection,
    index: usize,
}

#[derive(Debug)]
pub enum SolverError {
    InvalidSize(String),
    InvalidHint(String),
}

pub struct Solver<T: Board<Item = Cell>> {
    board: T,
    row_solver_info: SolvingInfo<T>,
    column_solver_info: SolvingInfo<T>,
    cache: DistributeNumberCalculator,
}

impl<T: Board<Item = Cell>> Default for SolvingInfo<T> {
    fn default() -> Self {
        SolvingInfo {
            count_board: T::new(Vec2 { row: 0, column: 0 }, Cell::None),
            possibility_count: Vec::new(),
            possibilities: Vec::new(),
            given_hint: Vec::new(),
        }
    }
}

impl<T: Board<Item = Cell>> Solver<T> {
    fn validate_hints(size: usize, hints: &[Vec<usize>], is_row: bool) -> Result<(), SolverError> {
        let dimension = if is_row { "row" } else { "column" };

        // 각 힌트의 유효성 검증
        for (idx, hint) in hints.iter().enumerate() {
            // 힌트 합이 보드 크기를 초과하는지 검증
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
    ) -> Result<Self, SolverError> {
        // 보드 크기 검증
        if size.row == 0 || size.column == 0 {
            return Err(SolverError::InvalidSize(format!(
                "Invalid board size: {}x{}",
                size.row, size.column
            )));
        }

        // 힌트 유효성 검증
        Self::validate_hints(size.row, &row_hint, true)?;
        Self::validate_hints(size.column, &column_hint, false)?;

        let mut solver = Self {
            board: T::new(size, Cell::None),
            row_solver_info: SolvingInfo::default(), // 임시값
            column_solver_info: SolvingInfo::default(), // 임시값
            cache: DistributeNumberCalculator::new(),
        };

        solver.row_solver_info = solver.create_solving_info(size.column, row_hint, size);
        solver.column_solver_info = solver.create_solving_info(size.row, column_hint, size);

        Ok(solver)
    }

    fn create_solving_info(
        &mut self,
        size: usize,
        hints: Vec<Vec<usize>>,
        board_size: Vec2,
    ) -> SolvingInfo<T> {
        let mut possibility_count = Vec::with_capacity(size);
        let mut possibilities = Vec::with_capacity(size);

        for hint in hints.iter() {
            let count = self.cache.calc_distribute_count_line_hint(hint, size);

            possibility_count.push(count);

            let mut possibility = BitSet::with_capacity(count);
            for j in 0..count {
                possibility.insert(j);
            }
            possibilities.push(possibility);
        }

        SolvingInfo {
            count_board: T::new(board_size, Cell::None),
            possibility_count,
            possibilities,
            given_hint: hints,
        }
    }

    fn solve_line(&mut self, line: Line) -> Result<(), SolverError> {
        let solving_info = match line.direction {
            LineDirection::Row => &mut self.row_solver_info,
            LineDirection::Column => &mut self.column_solver_info,
        };

        let length = match line.direction {
            LineDirection::Row => self.board.size().column,
            LineDirection::Column => self.board.size().row,
        };

        let possibilities = &mut solving_info.possibilities[line.index];
        let mut remove_possibility = BitSet::with_capacity(possibilities.len());
        let mut remove_possibility_count = 0;
        let mut new_line = vec![Cell::None; length];

        for possibility_index in possibilities.iter() {
            let indexed_line = self
                .cache
                .calc_distribute_number_line_hint(
                    &solving_info.given_hint[line.index],
                    length,
                    possibility_index,
                )
                .map_err(|e| SolverError::InvalidHint(e.to_string()))?;

            let iter_mut_check: Box<dyn Iterator<Item = &Cell>> = match line.direction {
                LineDirection::Row => Box::new(self.board.iter_row(line.index)),
                LineDirection::Column => Box::new(self.board.iter_column(line.index)),
            };

            if indexed_line
                .iter()
                .zip(iter_mut_check)
                .any(|(cell, indexed_cell)| *cell | *indexed_cell == Cell::Crash)
            {
                remove_possibility.insert(possibility_index);
                remove_possibility_count += 1;

                continue;
            }

            new_line
                .iter_mut()
                .zip(indexed_line.iter())
                .for_each(|(cell, &indexed_cell)| *cell = *cell | indexed_cell);
        }

        solving_info.possibility_count[line.index] -= remove_possibility_count;
        for index in remove_possibility.iter() {
            possibilities.remove(index);
        }

        let iter_mut_update: Box<dyn Iterator<Item = &mut Cell>> = match line.direction {
            LineDirection::Row => Box::new(self.board.iter_row_mut(line.index)),
            LineDirection::Column => Box::new(self.board.iter_column_mut(line.index)),
        };

        new_line
            .iter()
            .zip(iter_mut_update)
            .filter(|(new, _old)| **new != Cell::Crash)
            .for_each(|(new, old)| *old = *old | *new);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Cell::*;

    #[test]
    fn test_calc_distribute_number_line_hint_1() {
        let mut calculator = DistributeNumberCalculator::new();
        let result = calculator.calc_distribute_number_line_hint(&vec![2, 2], 7, 0);
        assert_eq!(
            result,
            Ok(vec![Block, Block, Blank, Block, Block, Blank, Blank])
        );
    }

    #[test]
    fn test_calc_distribute_number_line_hint_2() {
        let mut calculator = DistributeNumberCalculator::new();
        let result = calculator.calc_distribute_number_line_hint(&vec![2, 3, 3], 10, 0);
        assert_eq!(
            result,
            Ok(vec![
                Block, Block, Blank, Block, Block, Block, Blank, Block, Block, Block
            ])
        );
    }

    #[test]
    fn test_calc_distribute_number_line_hint_high_index() {
        let mut calculator = DistributeNumberCalculator::new();
        let result = calculator.calc_distribute_number_line_hint(&vec![2, 2], 10, 10);
        assert_eq!(
            result,
            Ok(vec![
                Blank, Block, Block, Blank, Blank, Blank, Blank, Blank, Block, Block
            ])
        );
    }

    #[test]
    fn test_solve_line() {
        use crate::board::BoardVec;
        let mut solver: Solver<BoardVec<Cell>> = Solver::new(
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
        )
        .unwrap();

        for row in 0..solver.board.size().row {
            solver
                .solve_line(Line {
                    direction: LineDirection::Row,
                    index: row,
                })
                .unwrap();
        }

        for col in 0..solver.board.size().column {
            solver
                .solve_line(Line {
                    direction: LineDirection::Column,
                    index: col,
                })
                .unwrap();
        }
        for row in 0..solver.board.size().row {
            solver
                .solve_line(Line {
                    direction: LineDirection::Row,
                    index: row,
                })
                .unwrap();
        }

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

        for row in 0..solver.board.size().row {
            for col in 0..solver.board.size().column {
                println!("{}, {}", row, col);
                assert_eq!(
                    solver.board.value(Vec2 { row, column: col }),
                    &expected_board[row][col]
                );
            }
        }
    }
}
