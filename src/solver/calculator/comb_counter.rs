use std::collections::HashMap;

type T = usize;
pub struct CombCounter {
    cache: HashMap<(T, usize), T>,
}

impl CombCounter {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn calc_comb_count(&mut self, amount: T, count: usize) -> T {
        if let Some(&result) = self.cache.get(&(amount, count)) {
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

        self.cache.insert((amount, count), result);
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
