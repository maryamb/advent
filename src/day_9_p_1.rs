use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
// use crate::io::Error;

fn read_file(file_path: &str) -> Lines<BufReader<File>> {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
      Err(why) => panic!("couldn't open {}: {}", file_path, why),
      Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);
  reader.lines()
}

fn read_line(line: &str) -> Vec<i32> {
  line.split(' ').filter_map(|w| w.parse().ok()).collect()
}

fn rec_deltas(input: &Vec<i32>) -> Option<i32> {
  // println!("vector is {:?}", input);
  let mut sums: i32 = 0_i32;
  let mut buffer = input.clone();
  loop {
    if buffer.iter().all(|num| *num == 0) {
      return Some(sums);
    }
    sums += *buffer.last()?;
    buffer = buffer[1..].iter().zip(buffer[..buffer.len() - 1].iter()).map(|(b, a)| b - a).collect();
  }
}

fn rec_deltas_p_2(input: &Vec<i32>) -> i32 {
  // println!("vector is {:?}", input);
  let mut buffer = input.clone();
  let mut firsts: Vec<i32> = Vec::new();
  loop {
    if buffer.iter().all(|num| *num == 0) {
      firsts.push(0);
      break;
    }
    firsts.push(*buffer.first().expect(""));
    buffer = buffer[1..].iter().zip(buffer[..buffer.len() - 1].iter()).map(|(b, a)| b - a).collect();
  }
  // println!("Firsts: {:?}", firsts);
  firsts.iter().rev().fold(0_i32, |acc, p_sum| -acc + p_sum)
}


fn main() {
  // let ex = "10  13  16  21  30  45";
  // println!("example for {ex} results in {:?}", rec_deltas_p_2(&read_line(ex)));
  let file_path = "data/d9.txt";
  let all_sums = read_file(file_path)
    .map(|line| read_line(line.expect("").as_str()))
    .map(|line_vec| rec_deltas_p_2(&line_vec))
    .fold(0_i32, |acc, line_sum| acc + line_sum);
  println!("{:?}", all_sums);
}