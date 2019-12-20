// Get the puzzle input! :P

#[cfg(test)]
mod tests {
    use crate::input;
    #[test]
    fn input_read_file_to_string() {
        assert_eq!(input::read_file_to_string("input/test"), "Test file\n")
    }
}

use std::fs::File;
use std::io::prelude::*;

/// Read file at `path` into a String.
///
/// Panics if there's a problem.
pub fn read_file_to_string(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
