use std::env;
use std::time::Instant;

mod console;

fn main() {
    /*
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    let mut app = SolverApp::new(&args);
    app.unwrap().start();

    let duration = start.elapsed();

    println!(" {:?}", duration);

    // Rust에는 C#의 Console.ReadKey()와 정확히 일치하는 기능이 없습니다.
    // 대신, 사용자 입력을 기다리는 간단한 방법을 사용합니다.
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("입력을 읽는 데 실패했습니다.");
    */

    println!("Hello, World!");
}
