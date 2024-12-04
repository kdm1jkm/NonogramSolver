use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::RefCell;
use std::rc::Rc;

use super::display_handler::DisplayHandler;
use super::solver_display::SolverDisplay;
use super::{calculator::NumberDistributionCalculator, cell::Cell};
use bit_set::BitSet;

pub struct SolvingStrategy {
    possibilities: Vec<BitSet>,
    given_hint: Vec<Vec<usize>>,
    calculator: NumberDistributionCalculator,
    display: Option<Rc<RefCell<Box<dyn SolverDisplay>>>>,
}

impl DisplayHandler for SolvingStrategy {
    fn get_display(&self) -> Option<Rc<RefCell<Box<dyn SolverDisplay>>>> {
        self.display.as_ref().map(|d| Rc::clone(d))
    }
}

impl SolvingStrategy {
    pub fn new(size: usize, hints: Vec<Vec<usize>>, display: Box<dyn SolverDisplay>) -> Self {
        let mut calculator = NumberDistributionCalculator::new();
        let mut possibilities = Vec::with_capacity(size);

        for hint in hints.iter() {
            let count = calculator.calc_distribute_count_line_hint(hint, size);
            let mut possibility = BitSet::with_capacity(count);
            for j in 0..count {
                possibility.insert(j);
            }
            possibilities.push(possibility);
        }

        Self {
            possibilities,
            given_hint: hints,
            calculator,
            display: Some(Rc::new(RefCell::new(display))),
        }
    }

    pub fn solve_line(
        &mut self,
        line_index: usize,
        line_cells: &[Cell],
        line_length: usize,
    ) -> Result<Vec<Cell>, &'static str> {
        if !line_cells.contains(&Cell::Unknown) {
            return Ok(line_cells.to_vec());
        }

        let mut new_line = vec![Cell::Unknown; line_length];
        let mut indexed_line = Vec::new();
        let mut remove_possibility = BitSet::with_capacity(self.possibilities[line_index].len());
        let hint = &self.given_hint[line_index];

        let mut possibilities: Vec<usize> = self.possibilities[line_index].iter().collect();
        possibilities.shuffle(&mut thread_rng());

        let total_possibilities = possibilities.len();

        for (i, possibility_index) in possibilities.into_iter().enumerate() {
            self.update_progress((i + 1, total_possibilities));

            self.calculator.calc_distribute_number_line_hint(
                hint,
                line_length,
                possibility_index,
                &mut indexed_line,
            )?;

            if indexed_line
                .iter()
                .zip(line_cells.iter())
                .any(|(indexed_cell, cell)| (*indexed_cell | *cell) == Cell::Crash)
            {
                remove_possibility.insert(possibility_index);
                continue;
            }

            new_line
                .iter_mut()
                .zip(indexed_line.iter())
                .for_each(|(cell, &indexed_cell)| {
                    *cell = *cell | indexed_cell;
                });

            if new_line
                .iter()
                .zip(line_cells.iter())
                .all(|(new_cell, line_cell)| {
                    *line_cell != Cell::Unknown && *new_cell == Cell::Crash
                })
            {
                break;
            }
        }

        for index in remove_possibility.iter() {
            self.possibilities[line_index].remove(index);
        }

        Ok(new_line)
    }

    pub fn get_line_possibility_count(&self, line_index: usize) -> usize {
        self.possibilities[line_index].len()
    }
}
