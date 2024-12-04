use super::cell::Cell;
use crate::board::Board;
use std::sync::{Arc, RwLock};

pub struct BoardFormatter {
    board: Arc<RwLock<Board<Cell>>>,
    row_hints: Vec<Vec<usize>>,
    column_hints: Vec<Vec<usize>>,
}

impl BoardFormatter {
    pub fn new(
        board: Arc<RwLock<Board<Cell>>>,
        row_hints: Vec<Vec<usize>>,
        column_hints: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            board,
            row_hints,
            column_hints,
        }
    }

    pub fn to_string(&self) -> String {
        let board = self.board.read().unwrap();
        let capacity = board.size().row * board.size().column * 4 + board.size().row * 2 + 1000;
        let mut result = String::with_capacity(capacity);

        let max_row_hint_len = self
            .row_hints
            .iter()
            .map(|hint| hint.len())
            .max()
            .unwrap_or(0);

        let max_col_hint_len = self
            .column_hints
            .iter()
            .map(|hint| hint.len())
            .max()
            .unwrap_or(0);

        self.write_column_hints(&mut result, max_row_hint_len, max_col_hint_len);
        result.push('\n');
        self.write_board_content(&mut result, max_row_hint_len, &board);

        result
    }

    fn write_column_hints(
        &self,
        result: &mut String,
        max_row_hint_len: usize,
        max_col_hint_len: usize,
    ) {
        for i in 0..max_col_hint_len {
            result.push_str(&" ".repeat(max_row_hint_len * 4));
            for col_hint in &self.column_hints {
                if i < max_col_hint_len - col_hint.len() {
                    result.push_str("    ");
                } else {
                    let hint_index = i - (max_col_hint_len - col_hint.len());
                    result.push_str(&format!("{:<4}", col_hint[hint_index]));
                }
            }
            result.push('\n');
        }
    }

    fn write_board_content(
        &self,
        result: &mut String,
        max_row_hint_len: usize,
        board: &Board<Cell>,
    ) {
        for (row_index, row_hint) in self.row_hints.iter().enumerate() {
            let hint_str = row_hint
                .iter()
                .map(|h| h.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            result.push_str(&format!(
                "{:<width$}",
                hint_str,
                width = max_row_hint_len * 4
            ));

            // 한 줄의 셀들을 한 번에 처리
            for col_index in 0..board.size().column {
                let cell = board.value(crate::board::vec2::Vec2 {
                    row: row_index,
                    column: col_index,
                });
                result.push_str(&format!("{}{}", cell, cell));
            }
            result.push('\n');

            // 두 번째 줄
            result.push_str(&" ".repeat(max_row_hint_len * 4));
            for col_index in 0..board.size().column {
                let cell = board.value(crate::board::vec2::Vec2 {
                    row: row_index,
                    column: col_index,
                });
                result.push_str(&format!("{}{}", cell, cell));
            }
            result.push('\n');
        }
    }
}
