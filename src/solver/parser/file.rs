use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::SolverParser;
use crate::board::vec2::Vec2;

pub struct FileSolverParser<P: AsRef<Path>> {
    file_path: P,
}

impl<P: AsRef<Path>> FileSolverParser<P> {
    pub fn new(file_path: P) -> Self {
        Self { file_path }
    }
}

impl<P: AsRef<Path>> SolverParser for FileSolverParser<P> {
    fn parse(&self) -> Result<(Vec2, Vec<Vec<usize>>, Vec<Vec<usize>>), String> {
        let file = File::open(&self.file_path).map_err(|_| "Failed to open file.".to_string())?;
        let mut lines = io::BufReader::new(file).lines();

        // 첫 번째 줄: 행의 수와 열의 수
        let first_line = lines
            .next()
            .ok_or_else(|| "File is empty.".to_string())?
            .map_err(|_| "Failed to read the first line.".to_string())?;

        let first_line = first_line.trim_start_matches('\u{FEFF}'); // BOM 제거

        let mut dimensions = first_line.split_whitespace();
        let row_count: usize = dimensions
            .next()
            .ok_or_else(|| "Row count not found.".to_string())?
            .parse()
            .map_err(|_| "Failed to parse row count.".to_string())?;
        let column_count: usize = dimensions
            .next()
            .ok_or_else(|| "Column count not found.".to_string())?
            .parse()
            .map_err(|_| "Failed to parse column count.".to_string())?;

        // 행 힌트
        let mut row_hints = Vec::with_capacity(row_count);
        for _ in 0..row_count {
            let hint_line = lines
                .next()
                .ok_or_else(|| "Not enough row hints.".to_string())?
                .map_err(|_| "Failed to read row hint.".to_string())?;
            let hints = hint_line
                .split_whitespace()
                .map(|s| {
                    s.parse()
                        .map_err(|_| "Failed to parse row hint.".to_string())
                })
                .collect::<Result<Vec<usize>, _>>()?;
            row_hints.push(hints);
        }

        // 열 힌트
        let mut column_hints = Vec::with_capacity(column_count);
        for _ in 0..column_count {
            let hint_line = lines
                .next()
                .ok_or_else(|| "Not enough column hints.".to_string())?
                .map_err(|_| "Failed to read column hint.".to_string())?;
            let hints = hint_line
                .split_whitespace()
                .map(|s| {
                    s.parse()
                        .map_err(|_| "Failed to parse column hint.".to_string())
                })
                .collect::<Result<Vec<usize>, _>>()?;
            column_hints.push(hints);
        }

        Ok((
            Vec2 {
                row: row_count,
                column: column_count,
            },
            row_hints,
            column_hints,
        ))
    }
}
