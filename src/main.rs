use clap::Parser;
use nonogram_solver::{
    display::ConsoleDisplay,
    solver::parser::{FileSolverParser, HtmlTableSolverParser, SolverParser},
};

#[derive(Parser)]
struct Args {
    input_path: String,

    #[arg(long, default_value_t = false)]
    html: bool,

    #[arg(short, long, default_value_t = 0)]
    interval: u64,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut solver = if args.html {
        HtmlTableSolverParser::new(
            &std::fs::read_to_string(&args.input_path)
                .map_err(|e| format!("Failed to read file: {}", e))?,
        )
        .create_solver(Box::new(ConsoleDisplay::new(args.interval)))
    } else {
        FileSolverParser::new(&args.input_path)
            .create_solver(Box::new(ConsoleDisplay::new(args.interval)))
    }?;

    solver
        .solve()
        .map_err(|e| format!("Failed to solve: {:?}", e))?;

    let result = solver.board.to_string();
    drop(solver);
    println!("{}", result);
    Ok(())
}
