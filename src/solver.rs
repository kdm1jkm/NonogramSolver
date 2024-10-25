mod cell;
use calculator::DistributeNumberCalculator;
use cell::Cell;
pub mod calculator;

#[derive(Eq, Hash, PartialEq, Debug)]
struct LineHint {
    hint_numbers: Vec<usize>,
    length: usize,
}

struct DistributeIterator {
    distribute_number: DistributeNumberCalculator,
    hint: LineHint,
    index: usize,
}

impl DistributeNumberCalculator {
    fn calc_distribute_number_line_hint(
        &mut self,
        hint: LineHint,
        index: usize,
    ) -> Result<Vec<Cell>, &'static str> {
        let mut result = Vec::with_capacity(hint.length);

        let distribute = self.calc_distribute_number(
            hint.length - hint.hint_numbers.iter().sum::<usize>() - hint.hint_numbers.len() + 1,
            hint.hint_numbers.len() + 1,
            index,
        )?;

        for i in 0..hint.hint_numbers.len() {
            result.append(&mut vec![Cell::Blank; distribute[i]]);
            result.append(&mut vec![Cell::Block; hint.hint_numbers[i]]);
            if i < hint.hint_numbers.len() - 1 {
                result.push(Cell::Blank);
            }
        }

        result.append(&mut vec![Cell::Blank; distribute[distribute.len() - 1]]);

        return Ok(result);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_distribute_number_line_hint_1() {
        use Cell::*;
        let mut calculator = DistributeNumberCalculator::new();
        let result = calculator.calc_distribute_number_line_hint(
            LineHint {
                hint_numbers: vec![2, 2],
                length: 7,
            },
            0,
        );
        assert_eq!(
            result,
            Ok(vec![Block, Block, Blank, Block, Block, Blank, Blank])
        );
    }

    #[test]
    fn test_calc_distribute_number_line_hint_high_index() {
        use Cell::*;
        let mut calculator = DistributeNumberCalculator::new();
        let result = calculator.calc_distribute_number_line_hint(
            LineHint {
                hint_numbers: vec![2, 2],
                length: 10,
            },
            10,
        );
        assert_eq!(
            result,
            Ok(vec![
                Blank, Block, Block, Blank, Blank, Blank, Blank, Blank, Block, Block
            ])
        );
    }
}

/*
pub fn solve_line<B: Board>(line: &mut [Cell], hint: LineHint) {
    let possibilities: Vec<Vec<Cell>> = self.calculated_possibilities[&line_info]
        .iter()
        .filter(|possibility| !Self::merge_line(&line, possibility).contains(&Cell::Crash))
        .cloned()
        .collect();

    self.calculated_possibilities
        .insert(line_info.clone(), possibilities.clone());

    let merged_possibility: Vec<Cell> = possibilities
        .into_iter()
        .reduce(Self::merge_line)
        .unwrap()
        .into_iter()
        .map(|cell| {
            if cell == Cell::Crash {
                Cell::None
            } else {
                cell
            }
        })
        .collect();

    let merged_line = Self::merge_line(&line, &merged_possibility);

    self.board.set_line(&line_info, &merged_line);

    SolveResult {
        change_pos: line
            .iter()
            .zip(merged_line.iter())
            .enumerate()
            .filter(|(_, (a, b))| a != b)
            .map(|(i, _)| i)
            .collect(),
    }
}

pub fn get_cached_length(&self) -> usize {
    self.calculated_possibilities
        .values()
        .map(|v| v.len())
        .sum()
}

pub fn is_map_clear(&self) -> bool {
    !self
        .board
        .any(|cell| matches!(cell, Cell::None | Cell::Crash))
}

pub fn count_determined(&self) -> usize {
    (0..self.board.get_row())
        .map(|i| {
            self.board
                .get_row_line(i)
                .filter(|&cell| matches!(cell, Cell::Blank | Cell::Block))
                .count()
        })
        .sum()
}

fn merge_line(a: &[Cell], b: &[Cell]) -> Vec<Cell> {
    assert_eq!(a.len(), b.len(), "List size must be same");
    a.iter().zip(b.iter()).map(|(&a, &b)| a | b).collect()
}
*/
