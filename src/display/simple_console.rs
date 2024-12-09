use std::{thread::sleep, time::Duration};

use crate::solver::solver_display::SolverDisplay;

pub struct SimpleConsoleDisplay {
    interval_ms: u64,
}

impl SimpleConsoleDisplay {
    pub fn new(interval_ms: u64) -> Self {
        Self { interval_ms }
    }

    pub fn new_with_default() -> Self {
        Self::new(0)
    }
}

impl SolverDisplay for SimpleConsoleDisplay {
    fn change_state(&mut self, state: crate::solver::solver_display::SolverState) {
        match state {
            crate::solver::solver_display::SolverState::Loading(message) => {
                println!("Loading... {}", message)
            }
            crate::solver::solver_display::SolverState::Idle => println!("Ready to solve!"),
            crate::solver::solver_display::SolverState::Solving(solving_context) => {
                print!("{esc}[K", esc = 27 as char);
                println!(
                    "{} ... {}",
                    solving_context.line,
                    solving_context.line_waiting.len()
                );
                sleep(Duration::from_millis(self.interval_ms));
            }
            crate::solver::solver_display::SolverState::Solved => println!("Solved!"),
        }
    }

    fn update_progress(&mut self, progress: (usize, usize)) {
        if progress.0 % 9991 != 0 && progress.0 != progress.1 {
            return;
        }
        let progress_length = 40;
        let progress_count =
            (((progress.0 as f64) / (progress.1 as f64)) * (progress_length as f64)) as usize;

        print!(
            "[{}{}] {}/{}\r",
            "#".repeat(progress_count),
            " ".repeat(progress_length - progress_count),
            progress.0,
            progress.1
        );
    }
}
