use nonogram_solver::console::create_solver_from_file;

fn main() {
    let mut solver = create_solver_from_file("./sample/data3.txt").unwrap();

    println!("{}", solver.to_string());

    solver.solve().unwrap();
    println!("{}", solver.to_string());
}
