use crate::board::Board;

use super::{cell::Cell, types::Line};

#[derive(Clone)]
pub enum SolverState {
    Loading(String),
    Idle,
    Solving(SolvingContext),
    Solved,
}

#[derive(Clone)]
pub struct SolvingContext {
    pub board: Board<Cell>,
    pub line: Line,
    pub line_waiting: Vec<Line>,
}

pub trait SolverDisplay {
    fn change_state(&mut self, state: SolverState);
    fn update_progress(&mut self, progress: (usize, usize));
}
