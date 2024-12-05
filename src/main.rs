use clap::Parser;
use nonogram_solver::console::{create_solver_from_file, create_solver_from_html_table};

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
        create_solver_from_html_table(
            &std::fs::read_to_string(&args.input_path)
                .map_err(|e| format!("Failed to read file: {}", e))?,
            args.interval,
        )
    } else {
        create_solver_from_file(&args.input_path, args.interval)
    }?;

    solver
        .solve()
        .map_err(|e| format!("Failed to solve: {:?}", e))?;

    let result = solver.to_string();
    drop(solver);
    println!("{}", result);
    Ok(())
}
