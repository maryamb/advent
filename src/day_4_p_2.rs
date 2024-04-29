use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
// use std::collections::HashMap;
// use regex::Regex;


fn get_winning_numbers_in_line(line: &str) -> usize {
  let number_sets = line
  .split(':')
  .collect::<Vec<_>>()[1]
  .split('|')
  .collect::<Vec<&str>>();
  let winning_numbers = number_sets[0]
  .split(' ')
  .filter_map(|s| s.parse().ok()) 
  .collect::<HashSet<u32>>();
  number_sets[1]
  .split(' ')
  .filter_map(|s| s.parse().ok()) 
  .collect::<Vec<u32>>()
  .iter()
  .filter(|d| winning_numbers.contains(d))
  .collect::<Vec<&u32>>()
  .len()
}



fn accumulate_power_of_lines(file_path: &str) -> usize {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);

  let num_winning_nums_in_line: Vec<usize> = reader
  .lines()
  .map(|l| get_winning_numbers_in_line(&l.expect(""))).collect::<Vec<usize>>();
  let mut line_values: Vec<usize> = vec![0; num_winning_nums_in_line.len()];

  for i in (0..num_winning_nums_in_line.len()).rev() {
    // print!("i={:?}\t", i + 1);
    // print!("num_winning_nums_in_line[i]={:?}\t", num_winning_nums_in_line[i]);
    line_values[i] = line_values[i + 1..=i + num_winning_nums_in_line[i]].iter()
      .sum::<usize>() + 1;
    // println!("line_values[i]={:?}", line_values[i]);
  }

  line_values.iter().sum::<usize>()
}


fn main() {
  // let input_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
  println!("All of the points are  {:?}", accumulate_power_of_lines("data/d4_p1.txt"));
}