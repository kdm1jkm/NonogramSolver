mod file;
mod html;

pub use file::FileSolverParser;
pub use html::HtmlTableSolverParser;

use crate::board::vec2::Vec2;
use crate::solver::solver_display::SolverDisplay;
use crate::solver::{Solver, utils::SolverError};
pub trait SolverParser {
    fn parse(&self) -> Result<SolverParseResult, String>;

    fn create_solver(&self, display: Box<dyn SolverDisplay>) -> Result<Solver, String> {
        let result = self.parse()?;

        let size = result.board_size;
        let row_hints = result.row_hints;
        let column_hints = result.column_hints;

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
}

pub struct SolverParseResult {
    pub board_size: Vec2,
    pub row_hints: Vec<Vec<usize>>,
    pub column_hints: Vec<Vec<usize>>,
}
