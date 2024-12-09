use std::{error::Error, fmt::Display};

use super::{cell::Cell, types::Line};

#[derive(Debug)]
pub enum SolverError {
    InvalidBoardSize(usize, usize),
    InvalidInitialInfo(InvalidInfoError),
    InvalidSolvingState(SolvingError),
}

impl Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverError::InvalidBoardSize(width, height) => {
                write!(f, "Invalid board size: {}x{}", width, height)
            }
            SolverError::InvalidInitialInfo(e) => write!(f, "Invalid initial info: {:?}", e),
            SolverError::InvalidSolvingState(e) => write!(f, "Solving error: {:?}", e),
        }
    }
}

impl Error for SolverError {}

#[derive(Debug)]
pub struct InvalidInfoError {
    pub error_line: Line,
    pub size: usize,
    pub message: String,
}

#[derive(Debug)]
pub struct SolvingError {
    pub current_line: Vec<Cell>,
    pub calculating_line: Vec<Cell>,
    pub hint: Vec<usize>,
    pub error_line: Line,
    pub message: String,
}

impl From<SolverError> for String {
    fn from(e: SolverError) -> Self {
        e.to_string()
    }
}
