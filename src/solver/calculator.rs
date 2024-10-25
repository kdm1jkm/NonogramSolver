mod comb_counter;

pub struct DistributeNumberCalculator {
    comb_counter: comb_counter::CombCounter,
}

impl DistributeNumberCalculator {
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

        if self
            .comb_counter
            .calc_comb_count(amount as i32, count as i32)
            <= index as i32
        {
            return Err("Index out of range");
        }
        let mut left = amount;

        let mut counted_index = 0;

        for i in 0..(count - 2) {
            for j in 0..=left {
                let my_use = j;
                let their_use = left - j;

                let comb_count = self
                    .comb_counter
                    .calc_comb_count(their_use as i32, count as i32 - i as i32 - 1);
                counted_index += comb_count;

                if counted_index > index as i32 {
                    result[i] = my_use;
                    left -= my_use;
                    counted_index -= comb_count;
                    break;
                }
            }
        }

        result[count - 2] = index - counted_index as usize;
        result[count - 1] = left - (index - counted_index as usize);

        Ok(result)
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
        let mut distribute_number = DistributeNumberCalculator::new();
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
    fn test_calc_distribute_max_bound() {
        let result = calc_distribute_number_temp(5, 3, 20);
        assert_eq!(result, Ok(vec![5, 0, 0]));
    }

    #[test]
    fn test_index_out_of_range() {
        let result = calc_distribute_number_temp(5, 3, 50);
        assert_eq!(result, Err("Index out of range"));
    }

    #[test]
    fn test_whole_iterate() {
        let mut distribute_number = DistributeNumberCalculator::new();

        let amount = 20;
        let count = 5;

        let count = distribute_number
            .comb_counter
            .calc_comb_count(amount, count);

        let start_time = std::time::Instant::now();
        for i in 0..count as usize {
            distribute_number
                .calc_distribute_number(amount as usize, count as usize, i)
                .unwrap();
        }
        let duration = start_time.elapsed();
        println!("Time taken: {:?}", duration);
    }
}
