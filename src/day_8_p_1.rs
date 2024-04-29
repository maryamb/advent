use std::fs::File;
use std::io::{self, BufRead};
// use crate::io::Error;
use std::collections::HashMap;

fn tokenize(line: String) -> Vec<String> {
  line.split(&['=', ' ', ',', '(', ')']).filter(|w| !w.is_empty()).map(|w| w.to_string()).collect()
}

fn part_one(file_path: &str) {
  // let sequence = "LLR";
  let mut navigation_map: HashMap<String, (String, String)> = HashMap::new();
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
      Err(why) => panic!("couldn't open {}: {}", file_path, why),
      Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);
  let mut lines = reader.lines();

  let sequence = lines.next().expect("").expect("");
  println!("sequence: {:?}", sequence);

  for line in lines {
    let line = line.expect("");
    let tokenized_line = tokenize(line);
    // println!("tokenized_line: {:?}", tokenized_line);
    if tokenized_line.is_empty() {
      continue;
    }

    navigation_map.insert(
      tokenized_line[0].to_owned(), 
      (tokenized_line[1].to_owned(), tokenized_line[2].to_owned()));
  }
  // println!("navigation_map: {:?}", navigation_map);
  
  let mut index = 0;
  let mut count = 0;
  let seq_size = sequence.len();
  let left_or_right = |index: &usize| if sequence.chars().nth(*index).unwrap() == 'L' { 0_usize } else { 1_usize };
  let mut current = "AAA";
  loop {
    if count > 9999999 {
      break;
    }
    if current.ends_with('Z') {
      println!("current is {current} and count is {count}.");
    }
    count += 1;
    let ind = left_or_right(&index);
    let next = navigation_map.get(current).expect("");
    current = if ind == 0 { next.0.as_str() } else { next.1.as_str() };
    index = (index + 1) % seq_size;
  }
}

fn part_two() {
}


fn main() {
  let file_path = "data/d8.txt";
  part_one(&file_path);
  part_two();
}