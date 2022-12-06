use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
//
// See https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item=String>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(|l| l.expect("Couldn't read line?!")))
}

/// Checks for duplicate values in a slice
pub fn contains_duplicates<T: PartialEq>(slice: &[T]) -> bool {
    // see https://stackoverflow.com/a/46766782
    (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}