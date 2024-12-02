use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

type Vec3 = Vec<f64>;


#[derive(Clone, Debug)]
struct HalfLine {
  p: Vec3,
  v: Vec3,
}
fn get_intersection_point(line_1: &HalfLine, line_2: &HalfLine) -> Option<Vec3> {
  let eps = 1e-9; // Small epsilon for floating-point comparisons

  let det = line_1.v[0] * line_2.v[1] - line_1.v[1] * line_2.v[0];
  
  // Check if lines are parallel
  if det.abs() < eps {
      return None;
  }

  let dx = line_2.p[0] - line_1.p[0];
  let dy = line_2.p[1] - line_1.p[1];

  let t1 = (dx * line_2.v[1] - dy * line_2.v[0]) / det;
  let t2 = (dx * line_1.v[1] - dy * line_1.v[0]) / det;

  // Check if intersections are in the future
  if t1 < -eps || t2 < -eps {
      return None;
  }

  Some(vec![
      line_1.p[0] + t1 * line_1.v[0], 
      line_1.p[1] + t1 * line_1.v[1], 
      0.0
  ])
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
    let eps = 1e-9;
    point[0] >= min_point[0] - eps && point[0] <= max_point[0] + eps &&
    point[1] >= min_point[1] - eps && point[1] <= max_point[1] + eps
}

fn part_one(half_lines: Vec<HalfLine>, min_point: &Vec3, max_point: &Vec3) -> u32 {
  let mut sum : u32 = 0;
  for (i, line_1) in half_lines.iter().enumerate() {
    for line_2 in half_lines[i + 1..].iter() {
      // println!("Line 1: {:?}\nLine 2: {:?}", line_1, line_2);
      let intersect_point = get_intersection_point(line_1, line_2);
      // println!("intersect_point: {:?}\n", intersect_point);
      sum += match intersect_point {
        None => 0,
        Some(point) => match is_within(&point, min_point, max_point) {
          true => 1,
          false => 0
        }
      };
    }
  }
  return sum;
}


fn main() {
  let half_lines = read_input_file("data/d24.txt");
  let min_point: Vec3 = vec![200000000000000.0, 200000000000000.0, f64::MIN];
  let max_point: Vec3 = vec![400000000000000.0, 400000000000000.0, f64::MAX];
  // let min_point = vec![7., 7., f64::MIN];
  // let max_point = vec![27., 27., f64::MAX];
  let all_withins = part_one(half_lines, &min_point, &max_point);
  println!("{:?}", all_withins);
  
}