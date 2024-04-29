use std::fs::File;
use std::io::{self, BufRead};
// use crate::io::Error;
use std::collections::HashMap;
use num::integer::lcm;


fn tokenize(line: String) -> Vec<String> {
  line.split(&['=', ' ', ',', '(', ')']).filter(|w| !w.is_empty()).map(|w| w.to_string()).collect()
}

fn count_to_z_ending(
  word: &str, 
  navigation_map: &HashMap<String, (String, String)>,
  sequence: &str) -> u64 {
  let mut index = 0;
  let mut count = 0;
  let seq_size = sequence.len();
  let left_or_right = |index: &usize| if sequence.chars().nth(*index).unwrap() == 'L' { 0_usize } else { 1_usize };
  let mut current = word;
  loop {
    if current.ends_with('Z') {
      return count;
    }
    count += 1;
    let ind = left_or_right(&index);
    let next = navigation_map.get(current).expect("");
    current = if ind == 0 { next.0.as_str() } else { next.1.as_str() };
    index = (index + 1) % seq_size;
  }
}

fn part_two(file_path: &str) {
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
  let current: Vec<&str> = navigation_map.iter().filter(|(k, _)| k.ends_with('A')).map(|(k, _)| k.as_str()).collect();
  let counts: Vec<u64> = current.iter().map(|word| count_to_z_ending(word, &navigation_map, sequence.as_str())).collect();
  println!("{:?}", counts);
  let counts_gcd = counts[1..].iter().fold(counts[0], |acc, count| lcm(acc, *count));
  println!("counts lcm is {counts_gcd}");
}


fn main() {
  let file_path = "data/d8.txt";
  part_two(&file_path);
}