pub mod calculator;
pub mod cell;

use crate::board::{vec2::Vec2, Board};
use bit_set::BitSet;
use calculator::DistributeNumberCalculator;
use cell::Cell;
use std::{collections::HashSet, fmt::Write};

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

struct SolvingInfo {
    possibility_count: Vec<usize>,
    possibilities: Vec<BitSet>,
    given_hint: Vec<Vec<usize>>,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum LineDirection {
    Row,
    Column,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Line {
    pub direction: LineDirection,
    pub index: usize,
}

#[derive(Debug)]
pub enum SolverError {
    InvalidSize(String),
    InvalidHint(String),
}

pub struct Solver {
    board: Board<Cell>,
    row_solver_info: SolvingInfo,
    column_solver_info: SolvingInfo,
    cache: DistributeNumberCalculator,
    line_changed: HashSet<Line>,
}

impl Default for SolvingInfo {
    fn default() -> Self {
        SolvingInfo {
            possibility_count: Vec::new(),
            possibilities: Vec::new(),
            given_hint: Vec::new(),
        }
    }
}

impl Solver {
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
        Self::validate_hints(size.column, &row_hint, true)?;
        Self::validate_hints(size.row, &column_hint, false)?;

        let mut solver = Self {
            board: Board::new(size, Cell::None),
            row_solver_info: SolvingInfo::default(), // 임시값
            column_solver_info: SolvingInfo::default(), // 임시값
            cache: DistributeNumberCalculator::new(),
            line_changed: HashSet::new(),
        };

        solver.row_solver_info = solver.create_solving_info(size.column, row_hint);
        solver.column_solver_info = solver.create_solving_info(size.row, column_hint);

        solver.initialize_line_changed();

        Ok(solver)
    }

    fn initialize_line_changed(&mut self) {
        let size = self.board.size();

        for row in 0..size.row {
            self.line_changed.insert(Line {
                direction: LineDirection::Row,
                index: row,
            });
        }

        for column in 0..size.column {
            self.line_changed.insert(Line {
                direction: LineDirection::Column,
                index: column,
            });
        }
    }

    fn create_solving_info(&mut self, size: usize, hints: Vec<Vec<usize>>) -> SolvingInfo {
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
            possibility_count,
            possibilities,
            given_hint: hints,
        }
    }

    fn solve_line<F>(&mut self, line: Line, callback: F) -> Result<(), SolverError>
    where
        F: Fn(usize, usize),
    {
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

        let mut count = 0;
        for possibility_index in possibilities.iter() {
            count += 1;
            callback(count, solving_info.possibility_count[line.index]);

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

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        // 행 힌트의 최대 길이를 구합니다.
        let max_row_hint_len = self
            .row_solver_info
            .given_hint
            .iter()
            .map(|hint| hint.len())
            .max()
            .unwrap_or(0);

        // 열 힌트의 최대 길이를 구합니다.
        let max_col_hint_len = self
            .column_solver_info
            .given_hint
            .iter()
            .map(|hint| hint.len())
            .max()
            .unwrap_or(0);

        // 열 힌트를 전치하여 보드 위에 출력합니다.
        for i in 0..max_col_hint_len {
            write!(result, "{:<width$}", " ", width = max_row_hint_len * 4).unwrap(); // 행 힌트 공간
            for col_hint in &self.column_solver_info.given_hint {
                if i < max_col_hint_len - col_hint.len() {
                    write!(result, "    ").unwrap(); // 빈 공간
                } else {
                    let hint_index = i - (max_col_hint_len - col_hint.len());
                    write!(result, "{:<4}", col_hint[hint_index]).unwrap(); // 고정된 너비 사용
                }
            }
            writeln!(result, "").unwrap();
        }

        writeln!(result, "").unwrap(); // 빈 줄 추가

        // 각 행에 대해 행 힌트와 보드 상태를 출력합니다.
        for (row_index, row_hint) in self.row_solver_info.given_hint.iter().enumerate() {
            let hint_str = row_hint
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            write!(result, "{:<width$}", hint_str, width = max_row_hint_len * 4).unwrap(); // 힌트를 왼쪽 정렬하여 출력

            // 첫 번째 줄 출력
            for col_index in 0..self.board.size().column {
                let cell = self.board.value(Vec2 {
                    row: row_index,
                    column: col_index,
                });
                write!(result, "{}{}", cell, cell).unwrap(); // Display 구현 사용
            }
            writeln!(result, "").unwrap();

            // 두 번째 줄 출력
            write!(result, "{:<width$}", "", width = max_row_hint_len * 4).unwrap(); // 두 번째 줄의 행 힌트 공간
            for col_index in 0..self.board.size().column {
                let cell = self.board.value(Vec2 {
                    row: row_index,
                    column: col_index,
                });
                write!(result, "{}{}", cell, cell).unwrap(); // Display 구현 사용
            }
            writeln!(result, "").unwrap();
        }

        result
    }

    pub fn solve_one_step<F>(&mut self, callback: F) -> Result<usize, SolverError>
    where
        F: Fn(usize, usize),
    {
        if let Some(line) = self.get_next_line() {
            self.line_changed.remove(&line);
            let before: Vec<Cell> = match line.direction {
                LineDirection::Row => self.board.iter_row(line.index).cloned().collect(),
                LineDirection::Column => self.board.iter_column(line.index).cloned().collect(),
            };

            if before.iter().all(|cell| *cell != Cell::None) {
                return Ok(0);
            }

            self.solve_line(line, callback)?;

            let after: Vec<Cell> = match line.direction {
                LineDirection::Row => self.board.iter_row(line.index).cloned().collect(),
                LineDirection::Column => self.board.iter_column(line.index).cloned().collect(),
            };

            let mut changed: Vec<usize> = Vec::new();

            for i in 0..before.len() {
                if before[i] != after[i] {
                    changed.push(i);
                }
            }

            let direction = match line.direction {
                LineDirection::Row => LineDirection::Column,
                LineDirection::Column => LineDirection::Row,
            };

            for e in changed.iter() {
                self.line_changed.insert(Line {
                    direction,
                    index: *e,
                });
            }

            return Ok(changed.len());
        }

        Ok(0)
    }

    pub fn is_solved(&self) -> bool {
        self.board.iter_all().all(|cell| *cell != Cell::None)
    }

    pub fn to_string_board(&self) -> String {
        self.board.to_string()
    }

    pub fn get_next_line(&mut self) -> Option<Line> {
        if self.is_solved() {
            return None;
        }
        let result = self
            .line_changed
            .iter()
            .min_by_key(|line| match line.direction {
                LineDirection::Row => self.row_solver_info.possibility_count[line.index],
                LineDirection::Column => self.column_solver_info.possibility_count[line.index],
            })
            .map(|line| *line);

        if result.is_none() {
            self.initialize_line_changed();
            return self.get_next_line();
        }

        result
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
        )
        .unwrap();

        solver.solve(0).unwrap();

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
