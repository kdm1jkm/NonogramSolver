#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vec2<T = usize>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Clone,
{
    pub row: T,
    pub column: T,
}

impl<T> Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Clone,
{
    pub fn new(row: T, column: T) -> Self {
        Self { row, column }
    }
}

use std::ops::{Add, Mul, Sub};

impl<T> Add for Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            column: self.column - other.column,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            row: self.row * scalar,
            column: self.column * scalar,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Vec2::new(1, 2) + Vec2::new(3, 4), Vec2::new(4, 6));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Vec2::new(3, 4) - Vec2::new(1, 2), Vec2::new(2, 2));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec2::new(1, 2) * 3, Vec2::new(3, 6));
    }
}
