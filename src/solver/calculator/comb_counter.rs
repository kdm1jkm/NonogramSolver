type T = usize;
pub struct CombCounter {
    cache: Vec<Vec<Option<T>>>,
}

impl CombCounter {
    pub fn new() -> Self {
        Self {
            cache: vec![vec![None; 100]; 100], // Adjust the size as needed
        }
    }

    pub fn calc_comb_count(&mut self, amount: T, count: usize) -> T {
        if let Some(result) = self.cache.get(amount).and_then(|v| v.get(count)).and_then(|&v| v) {
            return result;
        }

        if amount == 0 {
            return 1;
        }

        let result = match count {
            ..1 => unreachable!(),
            1 => 1,
            2 => amount + 1,
            3.. => (0..=amount)
                .rev()
                .map(|x| self.calc_comb_count(x, count - 1))
                .sum(),
        };

        if let Some(row) = self.cache.get_mut(amount) {
            if let Some(cell) = row.get_mut(count) {
                *cell = Some(result);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_comb_count() {
        let mut comb_counter = CombCounter::new();
        let result = comb_counter.calc_comb_count(20, 5);
        assert_eq!(result, 10626);
    }

    #[test]
    fn test_calc_comb_count_2() {
        let mut comb_counter = CombCounter::new();
        let result = comb_counter.calc_comb_count(10, 1);
        assert_eq!(result, 1);
    }
}
