use std::fmt;
use std::fmt::Display;

mod generic_mod1;
mod generic_mod2;

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

fn main() {
    println!("{}", generic_mod1::print_int_details(8008));

    let foo = generic_mod2::Foo { x: "X", y: Point { x: 20, y: 30 } };

    println!("foo = {}, len = {}", foo, foo.len());
}
