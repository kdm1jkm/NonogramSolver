use super::{calculator::comb_counter, cell::Cell};

pub struct NumberDistributionCalculator {
    pub comb_counter: comb_counter::CombCounter,
}

impl Default for NumberDistributionCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl NumberDistributionCalculator {
    pub fn new() -> Self {
        Self {
            comb_counter: comb_counter::CombCounter::new(),
        }
    }
    pub fn calc_distribute_number(
        &mut self,
        amount: usize,
        count: usize,
        index: usize,
    ) -> Result<Vec<usize>, &'static str> {
        let mut result = vec![0; count];

        if self.comb_counter.calc_comb_count(amount, count) <= index {
            return Err("Index out of range");
        }
        let mut left = amount;

        let mut counted_index = 0;

        for i in 0..count - 2 {
            for j in 0..=left {
                let my_use = j;
                let their_use = left - j;

                let comb_count = self.comb_counter.calc_comb_count(their_use, count - i - 1);
                counted_index += comb_count;

                if counted_index > index {
                    result[i] = my_use;
                    left -= my_use;
                    counted_index -= comb_count;
                    break;
                }
            }
        }

        result[count - 2] = index - counted_index;
        result[count - 1] = left - (index - counted_index);

        Ok(result)
    }

    pub fn calc_distribute_count_line_hint(
        &mut self,
        hint_numbers: &[usize],
        length: usize,
    ) -> usize {
        self.comb_counter.calc_comb_count(
            length + 1 - hint_numbers.iter().sum::<usize>() - hint_numbers.len(),
            hint_numbers.len() + 1,
        )
    }

    pub fn calc_distribute_number_line_hint(
        &mut self,
        hint_numbers: &[usize],
        length: usize,
        index: usize,
        result: &mut Vec<Cell>,
    ) -> Result<(), &'static str> {
        result.clear();

        if hint_numbers.is_empty() {
            result.append(&mut vec![Cell::Blank; length]);
            return Ok(());
        }

        let distribute = self.calc_distribute_number(
            length + 1 - hint_numbers.iter().sum::<usize>() - hint_numbers.len(),
            hint_numbers.len() + 1,
            index,
        )?;

        for i in 0..hint_numbers.len() {
            result.append(&mut vec![Cell::Blank; distribute[i]]);
            result.append(&mut vec![Cell::Block; hint_numbers[i]]);
            if i < hint_numbers.len() - 1 {
                result.push(Cell::Blank);
            }
        }
        result.append(&mut vec![Cell::Blank; distribute[distribute.len() - 1]]);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn calc_distribute_number_temp(
        amount: usize,
        count: usize,
        index: usize,
    ) -> Result<Vec<usize>, &'static str> {
        let mut distribute_number = NumberDistributionCalculator::new();
        distribute_number.calc_distribute_number(amount, count, index)
    }

    #[test]
    fn test_calc_distribute_number_1() {
        let result = calc_distribute_number_temp(5, 3, 0);
        assert_eq!(result, Ok(vec![0, 0, 5]));
    }

    #[test]
    fn test_calc_distribute_number_2() {
        let result = calc_distribute_number_temp(5, 3, 1);
        assert_eq!(result, Ok(vec![0, 1, 4]));
    }

    #[test]
    fn test_calc_distribute_number_3() {
        let result = calc_distribute_number_temp(5, 3, 2);
        assert_eq!(result, Ok(vec![0, 2, 3]));
    }

    #[test]
    fn test_calc_distribute_number_4() {
        let result = calc_distribute_number_temp(5, 3, 3);
        assert_eq!(result, Ok(vec![0, 3, 2]));
    }

    #[test]
    fn test_calc_distribute_number_5() {
        let result = calc_distribute_number_temp(6, 4, 0);
        assert_eq!(result, Ok(vec![0, 0, 0, 6]));
    }

    #[test]
    fn test_calc_distribute_number_6() {
        let result = calc_distribute_number_temp(6, 4, 1);
        assert_eq!(result, Ok(vec![0, 0, 1, 5]));
    }

    #[test]
    fn test_calc_distribute_number_7() {
        let result = calc_distribute_number_temp(6, 4, 2);
        assert_eq!(result, Ok(vec![0, 0, 2, 4]));
    }

    #[test]
    fn test_calc_distribute_number_8() {
        let result = calc_distribute_number_temp(6, 4, 3);
        assert_eq!(result, Ok(vec![0, 0, 3, 3]));
    }

    #[test]
    fn test_calc_distribute_number_9() {
        let result = calc_distribute_number_temp(6, 4, 4);
        assert_eq!(result, Ok(vec![0, 0, 4, 2]));
    }

    #[test]
    fn test_calc_distribute_number_10() {
        let result = calc_distribute_number_temp(0, 4, 0);
        assert_eq!(result, Ok(vec![0, 0, 0, 0]));
    }

    #[test]
    fn test_calc_distribute_max_bound() {
        let result = calc_distribute_number_temp(5, 3, 20);
        assert_eq!(result, Ok(vec![5, 0, 0]));
    }

    #[test]
    fn test_index_out_of_range() {
        let result = calc_distribute_number_temp(5, 3, 50);
        assert_eq!(result, Err("Index out of range"));
    }
}
