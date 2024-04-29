
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;


fn get_rgb_count(substr: &str) -> (u32, u32, u32) {
  let red_re = Regex::new(r" (\d+) red").unwrap();
  let green_re = Regex::new(r" (\d+) green").unwrap();
  let blue_re = Regex::new(r" (\d+) blue").unwrap();
  let red_caps = red_re.captures(substr);
  let green_caps = green_re.captures(substr);
  let blue_caps = blue_re.captures(substr);
  let red_c = match red_caps {
    Some(cap) => cap[1].parse().unwrap(),
    None => 0,
  };
  let green_c = match green_caps {
    Some(cap) => cap[1].parse().unwrap(),
    None => 0,
  };
  let blue_c = match blue_caps {
    Some(cap) => cap[1].parse().unwrap(),
    None => 0,
  };
  (red_c, green_c, blue_c)
}


fn get_game_number(substr: &str) -> u32 {
  let re = Regex::new(r"Game (\d+): ").unwrap();
  let caps = re.captures(substr).unwrap();
  caps[1].parse().unwrap()
}


fn get_max_rgbs(rgbs: &Vec<(u32, u32, u32)>) -> (u32, u32, u32) {
  rgbs.iter().fold((0, 0, 0), |agg, rgb| (agg.0.max(rgb.0), agg.1.max(rgb.1), agg.2.max(rgb.2)))
}


fn is_source_smaller(source_rgb: &(u32, u32, u32), demand_rgb: &(u32, u32, u32)) -> bool {
  source_rgb.0 < demand_rgb.0 || source_rgb.1 < demand_rgb.1 || source_rgb.2 < demand_rgb.2
}


// Returns game number if all games satisfy source
fn process_line(line: &str, source_rgb: &(u32, u32, u32)) -> u32 {
  let demand_rgb = line.split(';')
  .map(|substr| get_rgb_count(substr))
  .fold((0, 0, 0), |agg, rgb| (agg.0.max(rgb.0), agg.1.max(rgb.1), agg.2.max(rgb.2)));
  // println!("demand is {:?}", demand_rgb);

  if is_source_smaller(source_rgb, &demand_rgb) {
    0
  } else {
    get_game_number(line)
  }
}


fn power_of_line(line: &str) -> u32 {
  let demand_rgb = line.split(';')
  .map(|substr| get_rgb_count(substr))
  .fold((0, 0, 0), |agg, rgb| (agg.0.max(rgb.0), agg.1.max(rgb.1), agg.2.max(rgb.2)));
  demand_rgb.0 * demand_rgb.1 * demand_rgb.2
}


fn accumulate_lines(file_path: &str, source_rgb: &(u32, u32, u32)) -> u32 {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);

  reader.lines().fold(0, |acc, line| acc + process_line(&line.unwrap(), source_rgb))
}


fn accumulate_power_of_lines(file_path: &str) -> u32 {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);

  reader.lines().fold(0, |acc, line| acc + power_of_line(&line.unwrap()))
}


fn main() {
  let input_str = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
  let rgbc = get_rgb_count(input_str);
  let game_number = get_game_number(input_str);
  let a_vec = vec![(1_u32, 4_u32, 2_u32), (7_u32, 2_u32, 3_u32)];
  let src = (12, 13, 14);
  let demand = (1, 3, 3);
  println!("{:?}", process_line(input_str, &src));
  println!("processed file results in {:?}", accumulate_lines("data/d2_p1.txt", &src));
  println!("Accumulate power of all lines {:?}", accumulate_power_of_lines("data/d2_p1.txt"));
}