extern crate colored;

use std::fmt::{Display, Formatter, Write};
use colored::Colorize;

#[derive(Clone, Copy)]
pub enum BoardState {
    Block,
    Blank,
    Crash,
    None,
}

pub struct Board {
    values: Vec<BoardState>,
    height: u32,
    width: u32,
}

pub struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    pub fn new(x: u32, y: u32) -> Pos {
        Pos { x, y }
    }
}

pub enum Direction {
    Vertical,
    Horizontal,
}

pub struct LineIter<'a> {
    board: &'a mut Board,
    direction: Direction,
    index: u32,
    i: u32,
}

impl<'a> LineIter<'a> {
    fn new(board: &mut Board, direction: Direction, index: u32) -> LineIter {
        LineIter {
            board,
            direction,
            index,
            i: 0,
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = BoardState;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = match self.direction {
            Direction::Horizontal => Pos {
                x: self.i,
                y: self.index,
            },
            Direction::Vertical => Pos {
                x: self.index,
                y: self.i,
            },
        };
        let item = *self.board.get_element(&pos)?;

        self.i += 1;

        Some(item)
    }
}

impl Board {
    pub fn new(width: u32, height: u32) -> Result<Board, &'static str> {
        if width == 0 || height == 0 {
            return Err("");
        }
        let size = width * height;
        Ok(Board {
            width,
            height,
            values: vec![BoardState::None; size as usize],
        })
    }

    pub fn get_element(&self, pos: &Pos) -> Option<&BoardState> {
        let index = self.get_index(pos)?;
        self.values.get(index)
    }

    pub fn get_element_mut(&mut self, pos: &Pos) -> Option<&mut BoardState> {
        let index = self.get_index(pos)?;
        self.values.get_mut(index)
    }

    fn get_index(&self, pos: &Pos) -> Option<usize> {
        if pos.x >= self.width || pos.y >= self.height {
            return None;
        }

        Some((pos.x + self.width * pos.y) as usize)
    }

    pub fn get_line(&mut self, direction: Direction, index: u32) -> LineIter {
        LineIter::new(self, direction, index)
    }
}

impl Display for BoardState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardState::Block => f.write_str(&"  ".on_white().to_string()),
            BoardState::Blank => f.write_str("  "),
            BoardState::Crash => f.write_str(&"  ".on_red().to_string()),
            BoardState::None => f.write_str(&"  ".on_bright_black().to_string()),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_str(&self.get_element(&Pos { x, y }).unwrap().to_string())?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}
