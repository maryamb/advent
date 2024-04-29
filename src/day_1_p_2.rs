
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn get_number_in_line(char_map: &HashMap<&str, u32>, line: &str) -> Option<u32> {
  let get_index = |(k, v): (&&str, &u32)| {
    let mut result = vec![]; 
    if let Some(key_found_index) = line.find(*k) {
      result.push((key_found_index, *v));
    }
    if let Some(key_found_index) = line.rfind(*k) {
      result.push((key_found_index, *v));
    }
    result
  };
  let mut digit_arr: Vec<(usize, u32)> = char_map.iter()
    .map(|kv| get_index(kv))
    .flatten()
    .collect::<Vec<(usize, u32)>>();
  digit_arr.sort_by(|a, b| a.0.cmp(&b.0));
  // println!("{:?}", digit_arr);
  Some(digit_arr.get(0)?.1 * 10 + digit_arr.last()?.1)
}

fn accumulate_lines(char_map: &HashMap<&str, u32>, file_path: &str) -> Option<u32> {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);

  reader.lines().fold(Some(0_u32), |acc, line| Some(acc? + get_number_in_line(&char_map, &line.ok()?)?))

}

fn main() {
  let char_map: HashMap<&str, u32> = HashMap::from([
    /* ("zero", 0_u32), */
    ("one", 1_u32),
    ("two", 2_u32),
    ("three", 3_u32),
    ("four", 4_u32),
    ("five", 5_u32),
    ("six", 6_u32),
    ("seven", 7_u32),
    ("eight", 8_u32),
    ("nine", 9_u32),
    ("0", 0_u32),
    ("1", 1_u32),
    ("2", 2_u32),
    ("3", 3_u32),
    ("4", 4_u32),
    ("5", 5_u32),
    ("6", 6_u32),
    ("7", 7_u32),
    ("8", 8_u32),
    ("9", 9_u32),
  ]);
  /*
  two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
  */
  let examples = vec![
    "27zpq4one",
  "xczj475",
"2two3seven9rshkhrjzlv2",
"onethree222",
"fgeight6threegkjcgzjsfxqksgqvhnrhqf",
"sixsix8twotwoone",
"five3qr3two",
"76zxcjmfq",
"xchtklzgtwo4lbvnsix",
"cffmlvmsvnlrtgkstmqsdone9mdlkdgpgeightwogd",
"3onethree",
"lrdtfive3six",
  ];
  for input_string in examples {
    println!("Example {:?}, Solution: {:?}", input_string, get_number_in_line(&char_map, input_string));
  }
  println!("Sum of all numbers in the file is {:?}", accumulate_lines(&char_map, "data/d1_p1.txt"));



  // let reverse_char_map: HashMap<String, u32> = char_map.iter().
    // map(|(key, value)| (key.chars().rev().collect(), *value))
        // .collect();
}