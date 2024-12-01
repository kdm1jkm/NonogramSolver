use crate::board::vec2::Vec2;
use crate::solver::{Solver, SolverError};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn create_solver_from_file<P: AsRef<Path>>(file_path: P) -> Result<Solver, String> {
    let file = File::open(file_path).map_err(|_| "Failed to open file.".to_string())?;
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

    // Solver 생성
    Solver::new(
        Vec2 {
            row: row_count,
            column: column_count,
        },
        row_hints,
        column_hints,
    )
    .map_err(|e| {
        format!(
            "Failed to create Solver: {}",
            match e {
                SolverError::InvalidSize(s) => format!("Invalid size: {}", s),
                SolverError::InvalidHint(s) => format!("Invalid hint: {}", s),
            }
        )
    })
}

impl Solver {
    pub fn solve(&mut self, interval_ms: u64) -> Result<(), SolverError> {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        while !self.is_solved() {
            print!("{esc}[H", esc = 27 as char);
            let line = self.get_next_line();
            println!("{}", self.to_string_board());
            if let Some(line) = line {
                print!(
                    "\n                                                        \r Current line: {} {}",
                    match line.direction {
                        crate::solver::LineDirection::Row => "row",
                        crate::solver::LineDirection::Column => "column",
                    },
                    line.index
                );
                let changed = self.solve_one_step(|a, b| {
                    if a % 1000 == 0 {
                        print!(
                            "                                                        \r Current line: {} {} ({:.2}%)",
                            match line.direction {
                                crate::solver::LineDirection::Row => "row",
                                crate::solver::LineDirection::Column => "column",
                            },
                            line.index,
                            (a as f64 / b as f64) * 100.0,
                        );
                    }
                })?;
                if changed > 0 {
                    thread::sleep(Duration::from_millis(interval_ms));
                }
            } else {
                break;
            }
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Solved!");
        println!("{}", self.to_string_board());
        Ok(())
    }
}
