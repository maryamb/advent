
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;


fn get_game_number(substr: &str) -> u32 {
  let re = Regex::new(r"Game (\d+): ").unwrap();
  let caps = re.captures(substr).unwrap();
  caps[1].parse().unwrap()
}


fn get_winning_numbers_in_line(line: &str) -> i32 {
  let number_sets = line
  .split(':')
  .collect::<Vec<_>>()[1]
  .split('|')
  .collect::<Vec<&str>>();
  let winning_numbers = number_sets[0]
  .split(' ')
  .filter_map(|s| s.parse().ok()) 
  .collect::<HashSet<u32>>();
  let appearing_numbers = number_sets[1]
  .split(' ')
  .filter_map(|s| s.parse().ok()) 
  .collect::<Vec<u32>>()
  .iter()
  .filter(|d| winning_numbers.contains(d))
  .collect::<Vec<&u32>>()
  .len() as i32;
  // println!("There are {appearing_numbers} matches.\n");
  if appearing_numbers >= 1 {
    2_i32.pow((appearing_numbers - 1).try_into().unwrap())
   } else { 0 }
}


fn power_of_line(line: &str) -> u32 {
  let demand_rgb = line.split(';')
  .map(|substr| get_rgb_count(substr))
  .fold((0, 0, 0), |agg, rgb| (agg.0.max(rgb.0), agg.1.max(rgb.1), agg.2.max(rgb.2)));
  demand_rgb.0 * demand_rgb.1 * demand_rgb.2
}


fn accumulate_power_of_lines(file_path: &str) -> i32 {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);

  reader.lines().fold(0, |acc, line| acc + get_winning_numbers_in_line(&line.unwrap()))
}


fn main() {
  let input_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
  println!("len of this line is {:?}", get_winning_numbers_in_line(input_str));
  println!("power of line is  {:?}", accumulate_power_of_lines("data/d4_p1.txt"));
}