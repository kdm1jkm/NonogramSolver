mod file;
mod html;

pub use file::FileSolverParser;
pub use html::HtmlTableSolverParser;

use crate::board::vec2::Vec2;
use crate::solver::solver_display::SolverDisplay;
use crate::solver::{Solver, SolverError};

pub trait SolverParser {
    fn parse(&self) -> Result<(Vec2, Vec<Vec<usize>>, Vec<Vec<usize>>), String>;
}

pub fn create_solver<P: SolverParser>(
    parser: P,
    display: Box<dyn SolverDisplay>,
) -> Result<Solver, String> {
    let (size, row_hints, column_hints) = parser.parse()?;

    Solver::new(size, row_hints, column_hints, display).map_err(|e| {
        format!(
            "Failed to create Solver: {}",
            match e {
                SolverError::InvalidSize(s) => format!("Invalid size: {}", s),
                SolverError::InvalidHint(s) => format!("Invalid hint: {}", s),
            }
        )
    })
}
