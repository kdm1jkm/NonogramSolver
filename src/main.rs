use nonogram_solver::console::create_solver_from_file;

fn main() {
    use std::env;

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("사용법: {} <파일 이름> <interval_ms>", args[0]);
        std::process::exit(1);
    }
    let input_filename = &args[1];
    let interval_ms: u64 = args[2].parse().expect("interval_ms는 숫자여야 합니다.");
    let mut solver = create_solver_from_file(input_filename).unwrap();

    solver.solve(interval_ms).unwrap();
}
