use regex::Regex;

use crate::board::vec2::Vec2;
use crate::solver::solver_display::{SolverDisplay, SolverState, SolvingState};
use crate::solver::{LineDirection, Solver, SolverError};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub fn create_solver_from_file<P: AsRef<Path>>(
    file_path: P,
    interval_ms: u64,
) -> Result<Solver, String> {
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
        Box::new(ConsoleSolverDisplay::new(interval_ms)),
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

pub fn create_solver_from_html_table(html_table: &str, interval_ms: u64) -> Result<Solver, String> {
    // 열 힌트 추출 (첫 번째 tr의 td들)
    let column_td_re = Regex::new(r#"<td data-row="-1" data-col="\d+"[^>]*>(.*?)</td>"#).unwrap();
    let number_re = Regex::new(r"<span>(\d+)</span>").unwrap();

    let mut column_hints = Vec::new();
    for td_caps in column_td_re.captures_iter(html_table) {
        let td_content = td_caps.get(1).unwrap().as_str();
        let numbers: Vec<usize> = number_re
            .captures_iter(td_content)
            .filter_map(|cap| cap.get(1).and_then(|m| m.as_str().parse().ok()))
            .collect();
        column_hints.push(numbers); // 빈 td도 빈 Vec으로 추가
    }

    // 행 힌트 추출 (tbody 내의 두 번째 tr부터)
    let tbody_re = Regex::new(r"<tbody>(.*?)</tbody>").unwrap();
    let first_td_div_re =
        Regex::new(r"<tr[^>]*?>.*?<td[^>]*?><div>((?:<span>\d+<\/span>)+)</div></td>").unwrap();

    let mut row_hints = Vec::new();
    if let Some(tbody_caps) = tbody_re.captures(html_table) {
        let tbody_content = tbody_caps.get(1).unwrap().as_str();

        for td_caps in first_td_div_re.captures_iter(tbody_content) {
            let div_content = td_caps.get(1).unwrap().as_str();
            let numbers: Vec<usize> = number_re
                .captures_iter(div_content)
                .filter_map(|cap| cap.get(1).and_then(|m| m.as_str().parse().ok()))
                .collect();
            row_hints.push(numbers);
        }
    }

    // 최종 결과 출력
    println!("\nFinal results:");
    println!("Column hints:");
    for hint in &column_hints {
        println!("{:?}", hint);
    }
    println!("Row hints:");
    for hint in &row_hints {
        println!("{:?}", hint);
    }
    println!("Column hints count: {}", column_hints.len());
    println!("Row hints count: {}", row_hints.len());

    if !row_hints.is_empty() && !column_hints.is_empty() {
        return Solver::new(
            Vec2 {
                row: row_hints.len(),
                column: column_hints.len(),
            },
            row_hints,
            column_hints,
            Box::new(ConsoleSolverDisplay::new(interval_ms)),
        )
        .map_err(|e| format!("Failed to create Solver: {:?}", e));
    }

    Err("Failed to parse HTML table".to_string())
}

pub struct ConsoleSolverDisplay {
    state_sender: Sender<Option<SolvingState>>,
    progress_sender: Sender<Option<(usize, usize)>>,
    is_new_screen: Arc<Mutex<bool>>,
    interval_ms: u64,
    state_thread: Option<thread::JoinHandle<()>>,
}

impl ConsoleSolverDisplay {
    pub fn new(interval_ms: u64) -> Self {
        let (state_sender, state_receiver) = mpsc::channel::<Option<SolvingState>>();
        let (progress_sender, progress_receiver) = mpsc::channel::<Option<(usize, usize)>>();

        let is_new_screen = Arc::new(Mutex::new(false));
        let is_new_screen_clone = Arc::clone(&is_new_screen);

        let state_thread = thread::spawn(move || {
            while let Ok(solving_state) = state_receiver.recv() {
                if let None = solving_state {
                    return;
                }

                let is_new_screen = is_new_screen_clone.lock().unwrap();
                if !*is_new_screen {
                    drop(is_new_screen);
                    continue;
                }
                let solving_state = solving_state.unwrap();
                print!("{esc}[H", esc = 27 as char);

                let board = solving_state.board.read().unwrap();

                let board_string = board.to_string();
                let board_lines: Vec<&str> = board_string.lines().collect();

                for (row_idx, line) in board_lines.iter().enumerate() {
                    print!("{esc}[K", esc = 27 as char);

                    if solving_state.line.direction() == LineDirection::Row
                        && row_idx == solving_state.line.index()
                    {
                        print!("\x1b[93m{}\x1b[0m", line);
                        print!(" \x1b[93m←\x1b[0m");
                    } else if solving_state.line.direction() == LineDirection::Column {
                        let chars: Vec<char> = line.chars().collect();
                        for (col_idx, ch) in chars.chunks(2).enumerate() {
                            if col_idx == solving_state.line.index() {
                                print!("\x1b[93m{}{}\x1b[0m", ch[0], ch[1]);
                            } else {
                                print!("{}{}", ch[0], ch[1]);
                            }
                        }
                    } else {
                        print!("{}  ", line);
                    }
                    println!();
                }

                print!("{esc}[K", esc = 27 as char);
                if solving_state.line.direction() == LineDirection::Column {
                    for i in 0..board.size().column {
                        if i == solving_state.line.index() {
                            print!("\x1b[93m↑ \x1b[0m");
                        } else {
                            print!("  ");
                        }
                    }
                }

                println!();
                println!();

                print!("{esc}[K", esc = 27 as char);
                solving_state.line_waiting.iter().take(10).for_each(|line| {
                    print!(
                        "{}{} ",
                        line.index() + 1,
                        match line.direction() {
                            LineDirection::Row => "R",
                            LineDirection::Column => "C",
                        }
                    )
                });
                if solving_state.line_waiting.len() > 10 {
                    print!(" ...{} more", solving_state.line_waiting.len() - 10);
                }
                println!();
                println!();

                drop(is_new_screen);

                while let Ok(progress) = progress_receiver.recv() {
                    if let Some(progress) = progress {
                        if progress.0 % 5001 != 0 && progress.0 != progress.1 {
                            continue;
                        }

                        let progress_length = 40;
                        let progress_count = (((progress.0 as f64) / (progress.1 as f64))
                            * (progress_length as f64))
                            as usize;

                        let is_new_screen = is_new_screen_clone.lock().unwrap();
                        if !*is_new_screen {
                            drop(is_new_screen);
                            continue;
                        }

                        print!("{esc}[K", esc = 27 as char);
                        print!(
                            "\r[{}{}] {}/{}",
                            "#".repeat(progress_count),
                            " ".repeat(progress_length - progress_count),
                            progress.0,
                            progress.1
                        );
                        drop(is_new_screen);
                    } else {
                        break;
                    }
                }
            }
        });

        Self {
            state_sender,
            progress_sender,
            is_new_screen: Arc::clone(&is_new_screen),
            interval_ms,
            state_thread: Some(state_thread),
        }
    }

    pub fn new_with_default() -> Self {
        Self::new(0)
    }
}

impl SolverDisplay for ConsoleSolverDisplay {
    fn change_state(&mut self, state: SolverState) {
        if matches!(state, SolverState::Solving(_)) {
            let mut is_new_screen = self.is_new_screen.lock().unwrap();
            if !*is_new_screen {
                *is_new_screen = true;
                print!("{esc}[?1049h {esc}[J {esc}[?25l", esc = 27 as char);
            }
            drop(is_new_screen);
        } else {
            let mut is_new_screen = self.is_new_screen.lock().unwrap();
            if *is_new_screen {
                *is_new_screen = false;
                print!("{esc}[J {esc}[?1049l {esc}[?25h", esc = 27 as char);
            }
            drop(is_new_screen);
        }

        self.progress_sender.send(None).unwrap();
        match state {
            SolverState::Loading(message) => println!("Loading... {}", message),
            SolverState::Idle => println!("Ready to solve!"),
            SolverState::Solving(solving_state) => {
                self.progress_sender.send(None).unwrap();
                self.state_sender.send(Some(solving_state)).unwrap();
                if self.interval_ms > 0 {
                    thread::sleep(Duration::from_millis(self.interval_ms));
                }
            }
            SolverState::Solved => println!("Solved!"),
            SolverState::Error(message) => println!("Error: {}", message),
        }
    }

    fn update_progress(&mut self, progress: (usize, usize)) {
        self.progress_sender.send(Some(progress)).unwrap();
    }
}

impl Drop for ConsoleSolverDisplay {
    fn drop(&mut self) {
        self.progress_sender.send(None).unwrap();
        self.state_sender.send(None).unwrap();

        if let Some(t) = self.state_thread.take() {
            t.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_solver_from_html_table() {
        let html_table = include_str!("../../sample/table/data1.txt");
        let result = create_solver_from_html_table(html_table, 0);
        assert!(
            result.is_ok(),
            "Failed to create solver: {:?}",
            result.err()
        );

        let result = result.unwrap().solve();
        assert!(result.is_ok(), "Failed to solve: {:?}", result.err());
    }
}
