use std::fmt::Display;

#[inline]
pub fn stringify_and_cat<T: Display, U: Display>(t: T, u: U) -> String {
    format!("{}{}", t, u)
}

pub fn stringify_length<T: Display>(t: T) -> usize {
    format!("{}", t).len()
}

