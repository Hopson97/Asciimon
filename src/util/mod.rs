pub mod maths;
pub mod vector;

pub use self::vector::Vector2D;

use std::io::{stdout, Write};

pub fn flush_stdout() {
    stdout()
        .flush()
        .expect("Could not buffer the terminal output!");
}
