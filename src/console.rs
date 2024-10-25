use std::fmt::Debug;

/*
pub struct InputRefiner<T, F, G, H>
where
    F: Fn() -> (),
    G: Fn() -> String,
    H: Fn(String) -> (T, bool),
{
    before_get: F,
    getter: G,
    parser: H,
}

impl<T, F, G, H> InputRefiner<T, F, G, H>
where
    F: Fn() -> (),
    G: Fn() -> String,
    H: Fn(String) -> (T, bool),
    T: Debug,
{
    pub fn new(before_get: F, getter: G, parser: H) -> Self {
        InputRefiner {
            before_get,
            getter,
            parser,
        }
    }

    pub fn get_value(&self) -> T {
        loop {
            (self.before_get)();
            let input = (self.getter)();
            let (value, is_success) = (self.parser)(input);
            if is_success {
                return value;
            }
        }
    }
}

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;

use crate::solver::{Cell, Direction, LineInfo, Solver};

pub struct SolverApp {
    delay: u64,
    width: usize,
    height: usize,
    solver: Solver,
}

impl SolverApp {
    pub fn new(args: &[String]) -> io::Result<Self> {
        let file = if args.len() > 1 {
            &args[1]
        } else {
            println!("파일 이름을 입력해주세요:");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim()
        };

        let delay = if args.len() > 2 {
            args[2].parse().unwrap_or(0)
        } else {
            println!("딜레이를 입력해주세요 (밀리초):");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().parse().unwrap_or(0)
        };

        let file = File::open(file)?;
        let lines: Vec<String> = io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .collect();

        let meta_info: Vec<usize> = lines[0]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let width = meta_info[0];
        let height = meta_info[1];

        if lines.len() != width + height + 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "파일이 유효하지 않습니다.",
            ));
        }

        let converted_content: Vec<Vec<i32>> = lines[1..]
            .iter()
            .map(|s| s.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect();

        let horizontal_info = converted_content[0..height].to_vec();
        let vertical_info = converted_content[height..].to_vec();

        let solver = Solver::new(width, height, vertical_info, horizontal_info);

        Ok(SolverApp {
            delay,
            width,
            height,
            solver,
        })
    }

    pub fn start(&mut self) -> io::Result<()> {
        let mut works: VecDeque<LineInfo> = self
            .lines()
            .map(|line| (line, self.solver.get_number_of_cases(&line)))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(line, _)| line)
            .collect();

        execute!(io::stdout(), Hide)?;
        execute!(io::stdout(), Clear(ClearType::All))?;

        let x = (crossterm::terminal::size()?.0 as usize - self.width * 2) / 2;
        let y = (crossterm::terminal::size()?.1 as usize - self.height) / 2;

        let is_drawable = x > 0 && y > 1;

        if is_drawable {
            self.print_solver(x, y)?;
        } else {
            println!("그릴 수 없습니다.");
        }

        while !works.is_empty() {
            let line_info = works.pop_front().unwrap();

            execute!(io::stdout(), MoveTo(0, if is_drawable { 0 } else { 1 }))?;

            let count_determined = self.solver.count_determined();
            let total = self.width * self.height;
            print!(
                "\r캐시됨: {}  메모리: {}MB  {}/{}  {}%  {}/{}",
                self.solver.get_cached_length(),
                self.solver.get_memory_usage() / 1024 / 1024,
                count_determined,
                total,
                count_determined * 100 / total,
                line_info.index,
                line_info.direction
            );

            if is_drawable {
                thread::sleep(Duration::from_millis(self.delay));
            }

            let result = self.solver.solve_line(&line_info);

            if result.change_count == 0 {
                continue;
            }

            let other_direction = match line_info.direction {
                Direction::Vertical => Direction::Horizontal,
                Direction::Horizontal => Direction::Vertical,
            };

            let next_lines: Vec<LineInfo> = result
                .change_pos
                .iter()
                .map(|&pos| LineInfo::new(pos, other_direction))
                .collect();

            works.extend(next_lines);

            if self.solver.is_map_clear() {
                break;
            }

            if !is_drawable {
                continue;
            }

            let changed_poses = match line_info.direction {
                Direction::Vertical => result
                    .change_pos
                    .iter()
                    .map(|&pos| (line_info.index, pos))
                    .collect::<Vec<_>>(),
                Direction::Horizontal => result
                    .change_pos
                    .iter()
                    .map(|&pos| (pos, line_info.index))
                    .collect::<Vec<_>>(),
            };

            for (cx, cy) in changed_poses {
                execute!(io::stdout(), MoveTo((x + cx * 2) as u16, (y + cy) as u16))?;
                Self::print_cell(self.solver.get_cell(cx, cy))?;
            }
        }

        if is_drawable {
            self.print_solver(x, y)?;
        }

        execute!(io::stdout(), Show)?;
        Ok(())
    }

    fn lines(&self) -> impl Iterator<Item = LineInfo> {
        (0..self.height)
            .map(|i| LineInfo::new(i, Direction::Horizontal))
            .chain((0..self.width).map(|i| LineInfo::new(i, Direction::Vertical)))
    }

    fn print_solver(&self, x: usize, y: usize) -> io::Result<()> {
        for i in 0..self.solver.get_height() {
            execute!(io::stdout(), MoveTo(x as u16, (y + i) as u16))?;
            let line = self
                .solver
                .get_line(&LineInfo::new(i, Direction::Horizontal));
            for cell in line {
                Self::print_cell(cell)?;
            }
        }
        Ok(())
    }

    fn print_cell(cell: Cell) -> io::Result<()> {
        match cell {
            Cell::Block => {
                execute!(
                    io::stdout(),
                    SetBackgroundColor(Color::White),
                    SetForegroundColor(Color::White),
                    Print("  "),
                    ResetColor
                )?;
            }
            Cell::Blank => {
                execute!(io::stdout(), ResetColor, Print("  "))?;
            }
            Cell::None => {
                execute!(
                    io::stdout(),
                    SetBackgroundColor(Color::DarkGrey),
                    SetForegroundColor(Color::DarkGrey),
                    Print("  "),
                    ResetColor
                )?;
            }
            Cell::Crash => {
                execute!(
                    io::stdout(),
                    SetBackgroundColor(Color::Red),
                    SetForegroundColor(Color::Red),
                    Print("  "),
                    ResetColor
                )?;
            }
        }
        Ok(())
    }
}
*/
