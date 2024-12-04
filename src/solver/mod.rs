pub mod calculator;
pub mod cell;
pub mod solver_display;

use crate::board::{ vec2::Vec2, Board };
use bit_set::BitSet;
use calculator::DistributeNumberCalculator;
use cell::Cell;
use solver_display::{ SolverDisplay, SolverState, SolvingState };
use std::{ collections::HashSet, sync::{ Arc, RwLock } };

impl DistributeNumberCalculator {
    fn calc_distribute_number_line_hint(
        &mut self,
        hint_numbers: &[usize],
        length: usize,
        index: usize,
        result: &mut Vec<Cell>
    ) -> Result<(), &'static str> {
        result.clear();
        let distribute = self.calc_distribute_number(
            length + 1 - hint_numbers.iter().sum::<usize>() - hint_numbers.len(),
            hint_numbers.len() + 1,
            index
        )?;

        for i in 0..hint_numbers.len() {
            result.append(&mut vec![Cell::Blank; distribute[i]]);
            result.append(&mut vec![Cell::Block; hint_numbers[i]]);
            if i < hint_numbers.len() - 1 {
                result.push(Cell::Blank);
            }
        }

        result.append(&mut vec![Cell::Blank; distribute[distribute.len() - 1]]);

        Ok(())
    }

    fn calc_distribute_count_line_hint(&mut self, hint_numbers: &[usize], length: usize) -> usize {
        self.comb_counter.calc_comb_count(
            length + 1 - hint_numbers.iter().sum::<usize>() - hint_numbers.len(),
            hint_numbers.len() + 1
        )
    }
}

struct SolvingInfo {
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
        if (self.packed & (1 << 31)) == 0 { LineDirection::Row } else { LineDirection::Column }
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
    pub board: Arc<RwLock<Board<Cell>>>,
    row_solver_info: SolvingInfo,
    column_solver_info: SolvingInfo,
    cache: DistributeNumberCalculator,
    display: Option<Box<dyn SolverDisplay>>,
    pub line_changed: HashSet<Line>,
}

impl Default for SolvingInfo {
    fn default() -> Self {
        SolvingInfo {
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
                return Err(
                    SolverError::InvalidHint(
                        format!(
                            "{} {} hint sum {} (with spaces) exceeds size {}",
                            dimension,
                            idx,
                            spaces_needed,
                            size
                        )
                    )
                );
            }
        }

        Ok(())
    }

    pub fn new(
        size: Vec2,
        row_hint: Vec<Vec<usize>>,
        column_hint: Vec<Vec<usize>>,
        mut display: Box<dyn SolverDisplay>
    ) -> Result<Self, SolverError> {
        display.change_state(SolverState::Loading("Validate board size...".to_string()));

        if size.row == 0 || size.column == 0 {
            return Err(
                SolverError::InvalidSize(
                    format!("Invalid board size: {}x{}", size.row, size.column)
                )
            );
        }

        display.change_state(SolverState::Loading("Validate hints...".to_string()));

        Self::validate_hints(size.column, &row_hint, true)?;
        Self::validate_hints(size.row, &column_hint, false)?;

        let mut solver = Self {
            board: Arc::new(RwLock::new(Board::new(size, Cell::Unknown))),
            row_solver_info: SolvingInfo::default(),
            column_solver_info: SolvingInfo::default(),
            cache: DistributeNumberCalculator::new(),
            display: Some(display),
            line_changed: HashSet::new(),
        };

        if let Some(display) = solver.display.as_mut() {
            display.change_state(SolverState::Loading("Create initial info...".to_string()));
        }

        solver.row_solver_info = solver.create_solving_info(size.column, row_hint);
        solver.column_solver_info = solver.create_solving_info(size.row, column_hint);

        for r in 0..size.row {
            solver.line_changed.insert(Line::new(LineDirection::Row, r));
        }
        for c in 0..size.column {
            solver.line_changed.insert(Line::new(LineDirection::Column, c));
        }

        if let Some(display) = solver.display.as_mut() {
            display.change_state(SolverState::Idle);
        }

        Ok(solver)
    }

    fn create_solving_info(&mut self, size: usize, hints: Vec<Vec<usize>>) -> SolvingInfo {
        let mut possibilities = Vec::with_capacity(size);

        for hint in hints.iter() {
            let count = self.cache.calc_distribute_count_line_hint(hint, size);

            let mut possibility = BitSet::with_capacity(count);
            for j in 0..count {
                possibility.insert(j);
            }
            possibilities.push(possibility);
        }

        SolvingInfo {
            possibilities,
            given_hint: hints,
        }
    }

    pub fn to_string(&self) -> String {
        let board = self.board.read().unwrap();
        let capacity =
            board.size().row * board.size().column * 4 +
            board.size().row * 2 + // newlines
            1000; // 여유 공간
        let mut result = String::with_capacity(capacity);

        let max_row_hint_len = self.row_solver_info.given_hint
            .iter()
            .map(|hint| hint.len())
            .max()
            .unwrap_or(0);

        let max_col_hint_len = self.column_solver_info.given_hint
            .iter()
            .map(|hint| hint.len())
            .max()
            .unwrap_or(0);

        // 열 힌트 출력을 최적화
        for i in 0..max_col_hint_len {
            result.push_str(&" ".repeat(max_row_hint_len * 4));
            for col_hint in &self.column_solver_info.given_hint {
                if i < max_col_hint_len - col_hint.len() {
                    result.push_str("    ");
                } else {
                    let hint_index = i - (max_col_hint_len - col_hint.len());
                    result.push_str(&format!("{:<4}", col_hint[hint_index]));
                }
            }
            result.push('\n');
        }

        result.push('\n');

        // 보드 내용 출력을 최적화
        for (row_index, row_hint) in self.row_solver_info.given_hint.iter().enumerate() {
            let hint_str = row_hint
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            result.push_str(&format!("{:<width$}", hint_str, width = max_row_hint_len * 4));

            // 한 줄의 셀들을 한 번에 처리
            for col_index in 0..board.size().column {
                let cell = board.value(Vec2 {
                    row: row_index,
                    column: col_index,
                });
                result.push_str(&format!("{}{}", cell, cell));
            }
            result.push('\n');

            // 두 번째 줄
            result.push_str(&" ".repeat(max_row_hint_len * 4));
            for col_index in 0..board.size().column {
                let cell = board.value(Vec2 {
                    row: row_index,
                    column: col_index,
                });
                result.push_str(&format!("{}{}", cell, cell));
            }
            result.push('\n');
        }

        result
    }

    pub fn is_solved(&self) -> bool {
        self.board
            .read()
            .unwrap()
            .iter_all()
            .all(|cell| *cell != Cell::Unknown)
    }

    fn solve_line(&mut self, line: Line) -> Result<(), SolverError> {
        let board = self.board.read().unwrap();
        let line_cells: Vec<Cell> = match line.direction() {
            LineDirection::Row => board.iter_row(line.index()).cloned().collect(),
            LineDirection::Column => board.iter_column(line.index()).cloned().collect(),
        };
        drop(board);

        if !line_cells.contains(&Cell::Unknown) {
            return Ok(());
        }

        let length = line_cells.len();
        let mut new_line = vec![Cell::Unknown; length];
        let mut indexed_line = Vec::new();

        let solving_info = match line.direction() {
            LineDirection::Row => &self.row_solver_info,
            LineDirection::Column => &self.column_solver_info,
        };

        let mut remove_possibility = BitSet::with_capacity(
            solving_info.possibilities[line.index()].len()
        );
        let hint = &solving_info.given_hint[line.index()];

        let possibility_count = solving_info.possibilities[line.index()].len();
        let mut possibility_counted = 0;

        let line_waiting = self.line_order();

        if let Some(display) = self.display.as_mut() {
            display.change_state(
                SolverState::Solving(SolvingState {
                    board: Arc::clone(&self.board),
                    line,
                    line_waiting,
                })
            );
        }

        let possibilities: Vec<usize> = solving_info.possibilities[line.index()].iter().collect();
        for possibility_index in possibilities {
            possibility_counted += 1;
            if let Some(display) = self.display.as_mut() {
                display.update_progress((possibility_counted, possibility_count));
            }

            self.cache
                .calc_distribute_number_line_hint(
                    hint,
                    length,
                    possibility_index,
                    &mut indexed_line
                )
                .map_err(|e| SolverError::InvalidHint(e.to_string()))?;

            if
                indexed_line
                    .iter()
                    .zip(line_cells.iter())
                    .any(|(cell, indexed_cell)| (*cell | *indexed_cell) == Cell::Crash)
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
        }

        let solving_info = match line.direction() {
            LineDirection::Row => &mut self.row_solver_info,
            LineDirection::Column => &mut self.column_solver_info,
        };

        for index in remove_possibility.iter() {
            solving_info.possibilities[line.index()].remove(index);
        }

        let mut board = self.board.write().unwrap();
        let iter_mut: Box<dyn Iterator<Item = &mut Cell>> = match line.direction() {
            LineDirection::Row => Box::new(board.iter_row_mut(line.index())),
            LineDirection::Column => Box::new(board.iter_column_mut(line.index())),
        };

        iter_mut
            .zip(new_line.iter())
            .enumerate()
            .filter(|(_, (_, &new_cell))| new_cell != Cell::Crash)
            .filter(|(_, (_, &new_cell))| new_cell != Cell::Unknown)
            .filter(|(_, (board_cell, &new_cell))| **board_cell != new_cell)
            .for_each(|(index, (board_cell, &new_cell))| {
                self.line_changed.insert(
                    Line::new(
                        match line.direction() {
                            LineDirection::Row => LineDirection::Column,
                            LineDirection::Column => LineDirection::Row,
                        },
                        index
                    )
                );
                *board_cell = new_cell;
            });

        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), SolverError> {
        while let Some(line) = self.next_line_pop() {
            self.solve_line(line)?;
        }

        if let Some(display) = self.display.as_mut() {
            display.change_state(SolverState::Idle);
        }
        Ok(())
    }

    fn get_line_sort_key(&self, line: &Line) -> usize {
        self.get_line_possibility_count(line)
    }

    fn get_line_possibility_count(&self, line: &Line) -> usize {
        match line.direction() {
            LineDirection::Row => self.row_solver_info.possibilities[line.index()].len(),
            LineDirection::Column => self.column_solver_info.possibilities[line.index()].len(),
        }
    }

    fn next_line(&mut self) -> Option<Line> {
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

#[cfg(test)]
mod test {
    use crate::console::ConsoleSolverDisplay;

    use super::*;
    use Cell::*;

    #[test]
    fn test_calc_distribute_number_line_hint_1() {
        let mut calculator = DistributeNumberCalculator::new();
        let mut result = Vec::new();
        calculator.calc_distribute_number_line_hint(&vec![2, 2], 7, 0, &mut result).unwrap();
        assert_eq!(result, vec![Block, Block, Blank, Block, Block, Blank, Blank]);
    }

    #[test]
    fn test_calc_distribute_number_line_hint_2() {
        let mut calculator = DistributeNumberCalculator::new();
        let mut result = Vec::new();
        calculator.calc_distribute_number_line_hint(&vec![2, 3, 3], 10, 0, &mut result).unwrap();
        assert_eq!(
            result,
            vec![Block, Block, Blank, Block, Block, Block, Blank, Block, Block, Block]
        );
    }

    #[test]
    fn test_calc_distribute_number_line_hint_3() {
        let mut calculator = DistributeNumberCalculator::new();
        let mut result = Vec::new();
        calculator.calc_distribute_number_line_hint(&vec![2, 3, 4, 1], 30, 0, &mut result).unwrap();
        assert_eq!(
            result,
            vec![
                Block,
                Block,
                Blank,
                Block,
                Block,
                Block,
                Blank,
                Block,
                Block,
                Block,
                Block,
                Blank,
                Block,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank
            ]
        );
        let index_end = calculator.calc_distribute_count_line_hint(&vec![2, 3, 4, 1], 30) - 1;
        calculator
            .calc_distribute_number_line_hint(&vec![2, 3, 4, 1], 30, index_end, &mut result)
            .unwrap();

        assert_eq!(
            result,
            vec![
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Blank,
                Block,
                Block,
                Blank,
                Block,
                Block,
                Block,
                Blank,
                Block,
                Block,
                Block,
                Block,
                Blank,
                Block
            ]
        );
    }

    #[test]
    fn test_calc_distribute_number_line_hint_high_index() {
        let mut calculator = DistributeNumberCalculator::new();
        let mut result = Vec::new();
        calculator.calc_distribute_number_line_hint(&vec![2, 2], 10, 10, &mut result).unwrap();
        assert_eq!(
            result,
            vec![Blank, Block, Block, Blank, Blank, Blank, Blank, Blank, Block, Block]
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
                vec![2, 2, 2]
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
                vec![10]
            ],
            Box::new(ConsoleSolverDisplay::new_with_default())
        ).unwrap();

        solver.solve().unwrap();

        let expected_board = vec![
            vec![Block, Block, Blank, Blank, Block, Block, Blank, Blank, Block, Block],
            vec![Block, Block, Blank, Block, Block, Block, Blank, Blank, Block, Block],
            vec![Block, Block, Blank, Block, Block, Block, Blank, Block, Block, Block],
            vec![Block, Block, Blank, Blank, Block, Block, Blank, Block, Block, Block],
            vec![Block, Block, Blank, Blank, Block, Block, Blank, Blank, Block, Block],
            vec![Block, Block, Block, Blank, Block, Block, Blank, Blank, Block, Block],
            vec![Block, Block, Block, Blank, Block, Block, Block, Blank, Block, Block],
            vec![Block, Block, Blank, Blank, Block, Block, Block, Blank, Block, Block],
            vec![Block, Block, Blank, Blank, Block, Block, Blank, Blank, Block, Block],
            vec![Block, Block, Blank, Blank, Block, Block, Blank, Blank, Block, Block]
        ];

        for row in 0..solver.board.read().unwrap().size().row {
            for col in 0..solver.board.read().unwrap().size().column {
                println!("{}, {}", row, col);
                assert_eq!(
                    solver.board.read().unwrap().value(Vec2 { row, column: col }),
                    &expected_board[row][col]
                );
            }
        }
    }
}
