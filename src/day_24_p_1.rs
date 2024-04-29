use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::fmt;

// use crate::io::Error;
// use std::collections::HashMap;


fn read_input_file(file_path: &str) -> Matrix {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
      Err(why) => panic!("couldn't open {}: {}", file_path, why),
      Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);
  let lines = reader.lines();
  let mut tmp_matrix = Vec::new();
  for line in lines {
    tmp_matrix.push(line.expect("Line is non empty"));
  }

}


fn main() {
  // part_one();
  part_two();
}