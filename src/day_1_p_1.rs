
use std::fs::File;
use std::io::{self, BufRead};


fn get_first_digit(input: &str) -> Option<u32> {
  input.chars().find(|c| c.is_digit(10))?.to_digit(10)
}

fn get_last_digit(input: &str) -> Option<u32> {
  input.chars().rfind(|c| c.is_digit(10))?.to_digit(10)
}

fn get_line_number(input: &str) -> Option<u32> {
  Some(get_first_digit(input)? * 10 + get_last_digit(input)?)
}

fn accumulate_lines(file_path: &str) -> Option<u32> {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);

  reader.lines().fold(Some(0_u32), |acc, line| Some(acc? + get_line_number(&line.ok()?)?))

}

fn main() {
  let input_string = "abc123xyz";
  println!("First digit: {:?}", get_first_digit(input_string));
  println!("Last digit: {:?}", get_last_digit(input_string));
  println!("Sum of all numbers in the file is {:?}", accumulate_lines("data/d1_p1.txt"))
}