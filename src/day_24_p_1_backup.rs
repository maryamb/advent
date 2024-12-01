use std::fs::File;
use std::io::{self, BufRead};
use num::traits::float;
use regex::Regex;
use std::ops::Add;
use std::ops::Mul;


// use crate::io::Error;
// use std::collections::HashMap;

struct Vec3 {
  x: f32,
  y: f32,
  z: f32
}

// Implementing the Add trait for Vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Implementing the Mul trait for Vec3
impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
      self.x * rhs.x + self.y + rhs.y + self.z * rhs.z
    }
}

impl  {
    
}

#[derive(Clone, Debug)]
struct HalfLine {
  p: Vec3,
  v: Vec3,
}

impl Vec3 {
    fn norm(&self) -> f32 {
      (a_vec.x * a_vec.x + a_vec.y * a_vec.y + a_vec.z * a_vec.z).sqrt()
    }

    fn is_zero(epsilon: f32) -> bool {
      self::norm() < epsilon
    }
}


fn do_intersect(line_1: &HalfLine, line_2: &HalfLine) -> bool {
  /*
  p1 + v1 * t1 = p2 + v2 * t2
  
  p1.x + v1.x * t1 = p2.x + v2.x * t2
  p1.y + v1.y * t1 = p2.y + v2.y * t2

  (p1.x + p1.y) + (v1.x + v1.y) * t1 = (p2.x + p2.y) + (v2.x + v2.y) * t2
  */
  let p2 = line_2.p;
  let p1 = line_1.p;
  let p2_minus_p1 = Vec3(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
  let denum = -line_1.v * line_2.v;
  let numenator = line_1.v * p2_minus_p1;
  let m = numenator / denum;
  m >= 0
}


fn process_line(line: &String) -> HalfLine {
  let re = Regex::new(r"-?\d+").unwrap(); // Matches positive or negative integers
  let numbers: Vec<f64> = re.captures_iter(&line)
        .map(|cap| cap.get(0).unwrap().as_str().parse::<f64>().unwrap())
        .collect();
  // println!("{:?}", numbers);
  HalfLine {
    p: Vec3::new(numbers[0], numbers[1], numbers[2]), 
    v: Vec3::new(numbers[3], numbers[4], numbers[5])
  }
}


fn read_input_file(file_path: &str) -> Vec<HalfLine> {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
      Err(why) => panic!("couldn't open {}: {}", file_path, why),
      Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);
  let lines = reader.lines();
  let all_half_lines: Vec<HalfLine> = lines.into_iter().map(|line| process_line(&line.expect(""))).collect();
  all_half_lines
}

fn is_within(point: &na::Vector3<f64>, min_point: &Vec3<f64>, max_point: &Vec3<f64>) -> bool {
  point >= min_point && point <= max_point
}

fn part_one(half_lines: Vec<HalfLine>, min_point: &na::Vector3<f64>, max_point: &na::Vector3<f64>) -> u32 {
  let mut sum : u32 = 0;
  for (i, line_1) in half_lines.iter().enumerate() {
    for line_2 in half_lines[i + 1..].iter() {
      let intersect = do_intersect(line_1, line_2);
      let a = match intersect {
          None => 0,
          Some(intersect_point) => if is_within(&intersect_point, min_point, max_point) { 1 } else { 0 },
      };
      if a == 1 {
        println!("a:\n{:?}\nb:{:?}\nc:{:?}\n\n", line_1, line_2, intersect.expect(""));
      }
      sum += a;
    }
  }
  return sum;
}


fn main() {
  let half_lines = read_input_file("data/d24_ex.txt");
  let min_point = na::Vector3::new(7., 7., f64::MIN);
  let max_point = na::Vector3::new(27., 27., f64::MAX);
  let all_withins = part_one(half_lines, &min_point, &max_point);
  println!("{:?}", all_withins);
}