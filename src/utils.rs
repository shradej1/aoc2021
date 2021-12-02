use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// Reads the file with the given name from the puzzle input directory
/// ($CARGO_MANIFEST_DIR/inputs) and returns the contents in a string.
pub fn read_puzzle_input(filename: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs");
    path.push(filename);

    let mut f = File::open(path).unwrap();
    let mut str = String::new();
    f.read_to_string(&mut str).unwrap();
    str
}
