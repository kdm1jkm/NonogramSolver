use nonogram_solver::board::{self, Board};

fn main() {
    const BOARD_WIDTH: usize = 10;
    const BOARD_HEIGHT: usize = 10;
    const INITIAL_VALUE: usize = 0;

    let board_size = board::Vec2::new(BOARD_WIDTH, BOARD_HEIGHT);
    let board = board::default_board(board_size, INITIAL_VALUE);

    let row_line = board.get_row_line(0);
    let mut iter = row_line;
    while let Some(value) = iter.next() {
        print!("{:?} ", value);
    }
    println!();
    println!();

    for row in 0..board.get_row() {
        for col in 0..board.get_column() {
            print!("{:?} ", board.get_value(board::Vec2::new(col, row)));
        }
        println!();
    }
}
