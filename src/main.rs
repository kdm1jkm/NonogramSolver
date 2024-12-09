use clap::Parser;
use nonogram_solver::{
    display::{ConsoleDisplay, SimpleConsoleDisplay},
    solver::{
        parser::{FileSolverParser, HtmlTableSolverParser, SolverParser},
        solver_display::SolverDisplay,
    },
};

#[derive(Parser)]
struct Args {
    input_path: String,

    #[arg(long, default_value_t = false)]
    html: bool,

    #[arg(short, long, default_value_t = 0)]
    interval: u64,

    #[arg(long, default_value_t = false)]
    simple: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let display: Box<dyn SolverDisplay> = if args.simple {
        Box::new(SimpleConsoleDisplay::new(args.interval))
    } else {
        Box::new(ConsoleDisplay::new(args.interval))
    };

    let mut solver = if args.html {
        HtmlTableSolverParser::new(
            &std::fs::read_to_string(&args.input_path)
                .map_err(|e| format!("Failed to read file: {}", e))?,
        )
        .create_solver(display)
    } else {
        FileSolverParser::new(&args.input_path).create_solver(display)
    }?;

    solver
        .solve()
        .map_err(|e| format!("Failed to solve: {:?}", e))?;

    let result = solver.board.to_string();
    drop(solver);
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    fn solve_normal(filename: &str) -> Result<(), Box<dyn Error>> {
        let display = SimpleConsoleDisplay::new(0);
        let mut solver = FileSolverParser::new(filename).create_solver(Box::new(display))?;
        solver.solve()?;
        if !solver.is_solved() {
            return Err("Failed to solve".into());
        }
        Ok(())
    }

    fn solve_table(filename: &str) -> Result<(), Box<dyn Error>> {
        let display = SimpleConsoleDisplay::new(0);
        let content = std::fs::read_to_string(filename)?;
        let mut solver = HtmlTableSolverParser::new(&content).create_solver(Box::new(display))?;
        solver.solve()?;
        if !solver.is_solved() {
            return Err("Failed to solve".into());
        }
        Ok(())
    }

    #[test]
    fn test_solve_normal_1() {
        assert!(solve_normal("./sample/data1.txt").is_ok());
    }

    #[test]
    fn test_solve_normal_2() {
        assert!(solve_normal("./sample/data2.txt").is_ok());
    }

    #[test]
    fn test_solve_normal_3() {
        assert!(solve_normal("./sample/data3.txt").is_ok());
    }

    #[test]
    fn test_solve_normal_4() {
        assert!(solve_normal("./sample/data4.txt").is_ok());
    }

    #[test]
    fn test_solve_normal_5() {
        assert!(solve_normal("./sample/data5.txt").is_ok());
    }

    #[test]
    fn test_solve_table_1() {
        assert!(solve_table("./sample/table/data1.txt").is_ok());
    }

    #[test]
    fn test_solve_table_2() {
        assert!(solve_table("./sample/table/data2.txt").is_ok());
    }

    #[test]
    fn test_solve_table_3() {
        assert!(solve_table("./sample/table/data3.txt").is_ok());
    }

    #[test]
    fn test_solve_table_4() {
        assert!(solve_table("./sample/table/data4.txt").is_ok());
    }

    #[test]
    fn test_solve_table_5() {
        assert!(solve_table("./sample/table/data5.txt").is_ok());
    }
}
