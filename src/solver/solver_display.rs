use crate::board::Board;
use std::sync::{Arc, RwLock};

use super::{cell::Cell, Line};

#[derive(Clone)]
pub enum SolverState {
    Loading(String),
    Idle,
    Solving(SolvingState),
    Solved,
    Error(String),
}

#[derive(Clone)]
pub struct SolvingState {
    pub board: Arc<RwLock<Board<Cell>>>,
    pub line: Line,
    pub line_waiting: Vec<Line>,
}

pub trait SolverDisplay {
    fn change_state(&mut self, state: SolverState);
    fn update_progress(&mut self, progress: (usize, usize));
}
