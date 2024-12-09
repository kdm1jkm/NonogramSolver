use crate::solver::solver_display::{SolverDisplay, SolverState};
use crate::solver::types::LineDirection;
use std::thread;
use std::time::Duration;

pub struct ConsoleDisplay {
    is_new_screen: bool,
    interval_ms: u64,
}

impl ConsoleDisplay {
    pub fn new(interval_ms: u64) -> Self {
        Self {
            is_new_screen: false,
            interval_ms,
        }
    }

    pub fn new_with_default() -> Self {
        Self::new(0)
    }
}

impl SolverDisplay for ConsoleDisplay {
    fn change_state(&mut self, state: SolverState) {
        if matches!(state, SolverState::Solving(_)) {
            if !self.is_new_screen {
                self.is_new_screen = true;
                print!("{esc}[?1049h {esc}[J {esc}[?25l", esc = 27 as char);
            }
        } else if self.is_new_screen {
            self.is_new_screen = false;
            print!("{esc}[J {esc}[?1049l {esc}[?25h", esc = 27 as char);
        }

        match state {
            SolverState::Loading(message) => println!("Loading... {}", message),
            SolverState::Idle => println!("Ready to solve!"),
            SolverState::Solving(solving_state) => {
                print!("{esc}[H", esc = 27 as char);

                let board = solving_state.board;

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
                if self.interval_ms > 0 {
                    thread::sleep(Duration::from_millis(self.interval_ms));
                }
            }
            SolverState::Solved => println!("Solved!"),
        }
    }

    fn update_progress(&mut self, progress: (usize, usize)) {
        if !self.is_new_screen {
            return;
        }

        if progress.0 % 1991 != 0 && progress.0 != progress.1 {
            return;
        }

        let progress_length = 40;
        let progress_count =
            (((progress.0 as f64) / (progress.1 as f64)) * (progress_length as f64)) as usize;

        print!("{esc}[K", esc = 27 as char);
        print!(
            "\r[{}{}] {}/{}",
            "#".repeat(progress_count),
            " ".repeat(progress_length - progress_count),
            progress.0,
            progress.1
        );
    }
}
