use crate::board::Board;

use super::{cell::Cell, utils::Line};

#[derive(Clone)]
pub enum SolverState<'a> {
    Loading(String),
    Idle,
    Solving(SolvingState<'a>),
    Solved,
    Error(String),
}

#[derive(Clone)]
pub struct SolvingState<'a> {
    pub board: &'a Board<Cell>,
    pub line: Line,
    pub line_waiting: Vec<Line>,
}

pub trait SolverDisplay {
    fn change_state(&mut self, state: SolverState);
    fn update_progress(&mut self, progress: (usize, usize));
}
