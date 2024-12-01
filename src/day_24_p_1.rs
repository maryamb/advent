use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

type Vec3 = Vec<f64>;


#[derive(Clone, Debug)]
struct HalfLine {
  p: Vec3,
  v: Vec3,
}

fn get_intersection_time(line_1: &HalfLine, line_2: &HalfLine, index: usize) -> f64 {
  if line_1.v[index] == line_2.v[index] {
    return 0.;
  }
  (line_1.p[index] - line_2.p[index]) / (line_2.v[index] - line_1.v[index])
}

fn do_intersect(line_1: &HalfLine, line_2: &HalfLine) -> bool {
  let x_intersect_time = get_intersection_time(line_1, line_2, 0);
  let y_intersect_time = get_intersection_time(line_1, line_2, 1);
  x_intersect_time >= 0.0 && y_intersect_time == x_intersect_time
}

fn process_line(line: &String) -> HalfLine {
  let re = Regex::new(r"-?\d+").unwrap(); // Matches positive or negative integers
  let numbers: Vec<f64> = re.captures_iter(&line)
        .map(|cap| cap.get(0).unwrap().as_str().parse::<f64>().unwrap())
        .collect();
  // println!("{:?}", numbers);
  HalfLine {
    p: vec![numbers[0], numbers[1], numbers[2]], 
    v: vec![numbers[3], numbers[4], numbers[5]]
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

fn is_within(point: &Vec3, min_point: &Vec3, max_point: &Vec3) -> bool {
  point >= min_point && point <= max_point
}

fn part_one(half_lines: Vec<HalfLine>, min_point: &Vec3, max_point: &Vec3) -> u32 {
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
  // let half_lines = read_input_file("data/d24_ex.txt");
  // let min_point = Vec3::new(7., 7., f64::MIN);
  // let max_point = Vec3::new(27., 27., f64::MAX);
  // let all_withins = part_one(half_lines, &min_point, &max_point);
  // println!("{:?}", all_withins);
  let p1: Vec3 = vec![9., 13., 30.];
  let v1:  Vec3 = vec![-2., 1., -2.];
  let h1 = HalfLine{p: p1, v: v1};
  let p2: Vec3 = vec![318., 19., 22.];
  let v2: Vec3 = vec![-1., -1., -2.];
  let h2 = HalfLine{p: p2, v: v2};
  do_intersect(&h1, &h2);
}