use rand::rngs::ThreadRng;
use std::ops::Range;

pub trait Position<T> {
    fn position(&self) -> &crate::unit::Position<T>;
    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<T>, y_range: Range<T>);
}
