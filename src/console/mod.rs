use crate::board::vec2::Vec2;
use crate::solver::solver_display::{SolverDisplay, SolverState, SolvingState};
use crate::solver::{LineDirection, Solver, SolverError};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
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

pub struct ConsoleSolverDisplay {
    state_sender: Sender<Option<SolvingState>>,
    progress_sender: Sender<Option<(usize, usize)>>,
    is_new_screen: bool,
    interval_ms: u64,
    state_thread: Option<thread::JoinHandle<()>>,
    progress_thread: Option<thread::JoinHandle<()>>,
}

impl ConsoleSolverDisplay {
    pub fn new(interval_ms: u64) -> Self {
        let (state_sender, state_receiver) = mpsc::channel::<Option<SolvingState>>();
        let mutex = Arc::new(Mutex::new(()));
        let progress_mutex = Arc::clone(&mutex);

        let state_thread = thread::spawn(move || {
            while let Ok(solving_state) = state_receiver.recv() {
                if let None = solving_state {
                    return;
                }
                let solving_state = solving_state.unwrap();
                let guard = mutex.lock().unwrap();
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
                drop(guard);
            }
        });

        let (progress_sender, progress_receiver) = mpsc::channel::<Option<(usize, usize)>>();
        let progress_thread = thread::spawn(move || {
            while let Ok(progress) = progress_receiver.recv() {
                if let None = progress {
                    return;
                }
                let progress = progress.unwrap();
                if progress.0 % 543 != 0 {
                    continue;
                }

                let progress_length = 40;
                let progress_count = (((progress.0 as f64) / (progress.1 as f64))
                    * (progress_length as f64)) as usize;

                let guard = progress_mutex.lock().unwrap();
                print!("{esc}[K", esc = 27 as char);
                print!(
                    "\r[{}{}] {}/{}",
                    "#".repeat(progress_count),
                    " ".repeat(progress_length - progress_count),
                    progress.0,
                    progress.1
                );
                drop(guard);
            }
        });

        Self {
            state_sender,
            progress_sender,
            is_new_screen: false,
            interval_ms,
            state_thread: Some(state_thread),
            progress_thread: Some(progress_thread),
        }
    }

    pub fn new_with_default() -> Self {
        Self::new(0)
    }
}

impl SolverDisplay for ConsoleSolverDisplay {
    fn change_state(&mut self, state: SolverState) {
        if matches!(state, SolverState::Solving(_)) {
            if !self.is_new_screen {
                print!("{esc}[?1049h {esc}[J {esc}[?25l", esc = 27 as char);
                self.is_new_screen = true;
            }
        } else {
            if self.is_new_screen {
                // print!("{esc}[?1049l", esc = 27 as char);
                self.is_new_screen = false;
            }
        }

        match state {
            SolverState::Loading(message) => println!("Loading... {}", message),
            SolverState::Idle => println!("Idle"),
            SolverState::Solving(solving_state) => {
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
        self.state_sender.send(None).unwrap();
        self.progress_sender.send(None).unwrap();

        if let Some(t) = self.state_thread.take() {
            t.join().unwrap();
        }
        if let Some(t) = self.progress_thread.take() {
            t.join().unwrap();
        }

        print!("{esc}[?1049l", esc = 27 as char);
    }
}
