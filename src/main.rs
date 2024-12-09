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
