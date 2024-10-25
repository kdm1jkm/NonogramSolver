#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vec2 {
    pub row: usize,
    pub column: usize,
}

impl Vec2 {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

use std::ops::{Add, Mul, Sub};

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            column: self.column - other.column,
        }
    }
}

impl Mul<usize> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self {
        Self {
            row: self.row * scalar,
            column: self.column * scalar,
        }
    }
}

impl Mul<Vec2> for usize {
    type Output = Vec2;

    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 {
            row: self * vec.row,
            column: self * vec.column,
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

    #[test]
    fn test_mul_vec() {
        assert_eq!(3 * Vec2::new(1, 2), Vec2::new(3, 6));
    }
}
