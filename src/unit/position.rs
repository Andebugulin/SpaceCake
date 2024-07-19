#[derive(Debug, PartialEq)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T: Default> Default for Position<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}
