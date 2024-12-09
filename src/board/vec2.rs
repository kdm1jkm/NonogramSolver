#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vec2<T = usize>
where
    T: Copy + Clone,
{
    pub row: T,
    pub column: T,
}

impl<T> Vec2<T>
where
    T: Copy + Clone,
{
    pub fn new(row: T, column: T) -> Self {
        Self { row, column }
    }
}
