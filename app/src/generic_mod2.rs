use std::fmt::Display;
use std::fmt;

use sdk::generic;

pub struct Foo<T: Display, U: Display> {
    pub x: T,
    pub y: U,
}

impl<T: Display, U:Display> Display for Foo<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", generic::stringify_and_cat(&self.x, &self.y))
    }
}

impl<T: Display, U: Display> Foo<T, U> {
    pub fn len(&self) -> usize {
        generic::stringify_length(self)
    }
}

