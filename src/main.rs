use clap::Parser;
use nonogram_solver::console::{create_solver_from_file, create_solver_from_html_table};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 입력 파일 경로
    input_path: String,

    /// HTML 테이블 형식 사용
    #[arg(long, default_value_t = false)]
    html: bool,

    /// 업데이트 간격 (밀리초)
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
