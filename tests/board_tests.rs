use nonogram_solver::board::{self, Board};

#[test]
fn test_board_creation() {
    let board_size = board::Vec2::new(5, 5);
    let board = board::default_board(board_size, 0);
    assert_eq!(board.get_row(), 5);
    assert_eq!(board.get_column(), 5);
}

#[test]
fn test_board_get_set_value() {
    let board_size = board::Vec2::new(3, 3);
    let mut board = board::default_board(board_size, 0);
    let pos = board::Vec2::new(1, 1);
    board.set_value(pos, 5);
    assert_eq!(board.get_value(pos), 5);
}

#[test]
fn test_board_get_row_line() {
    let board_size = board::Vec2::new(3, 3);
    let mut board = board::default_board(board_size, 0);
    board.set_value(board::Vec2::new(1, 0), 1);
    board.set_value(board::Vec2::new(1, 1), 2);
    board.set_value(board::Vec2::new(1, 2), 3);
    let row_line: Vec<usize> = board.get_row_line(1).collect();
    assert_eq!(row_line, vec![1, 2, 3]);
}
