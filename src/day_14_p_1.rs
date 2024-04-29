use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
// use crate::io::Error;
// use std::collections::HashMap;


fn get_matrix(file_path: &str) -> Vec<String> {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
      Err(why) => panic!("couldn't open {}: {}", file_path, why),
      Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);
  let lines = reader.lines();
  let mut dish_matrix = Vec::new();
  for line in lines {
    dish_matrix.push(line.expect("Line is non empty"));
  }
  dish_matrix
}

fn get_col(dish_matrix: &Vec<String>, col: usize) -> Vec<char> {
  dish_matrix.iter().filter_map(|s| s.chars().nth(col)).collect()
}

fn get_square_rock_indices(col_vec: &Vec<char>) -> Vec<usize> {
  col_vec
      .iter()
      .enumerate()
      .filter_map(|(index, &character)| if character == '#' { Some(index) } else { None } )
      .collect()
}

fn get_rounds_in_col(col_vec: &Vec<char>) -> Vec<(usize, usize)> {
  let occurrences = |s, e| col_vec[s..e]
        .iter()
        .filter(|&&c| c == 'O')
        .count();
  let squares = get_square_rock_indices(col_vec);
  let starts = [vec![0usize], squares.iter().map(|a| a + 1).collect()].concat();
  let ends = [squares.clone(), vec![col_vec.len()]].concat();
  let intervals = zip(starts, ends);
  intervals.filter_map(|(s, e)| Some((s, occurrences(s, e)))).collect()
}

fn interval_worth(start_ind: usize, num: usize, total_rows: usize) -> i32 {
  let b = total_rows - start_ind;
  let a = b - num + 1;
  ((a + b) * num / 2) as i32
}

fn get_col_worth(rounds_in_col: &Vec<(usize, usize)>, total_rows: usize) -> i32 {
  rounds_in_col.iter().fold(0, |acc, (s, n)| acc + interval_worth(*s, *n, total_rows))
}

fn part_one() {
  let file_path = "data/d14.txt";
  let dish_matrix = get_matrix(&file_path);
  let third_col = get_col(&dish_matrix, 2);
  // println!("{:?}", get_square_rock_indices(&third_col));
  let r_in_c_3 = get_rounds_in_col(&third_col);
  println!("{:?}", r_in_c_3);
  println!("interval worth (0, 4, 10) {:?}", interval_worth(0, 4, 10));
  println!("get_col_worth {:?}", get_col_worth(&r_in_c_3, third_col.len()));

  let num_cols = dish_matrix[0].len();
  let total_rows = dish_matrix.len();
  let mut total_worth = 0;
  for col_number in 0..num_cols {
    let col_vec = get_col(&dish_matrix, col_number);
    let rounds_in_col = get_rounds_in_col(&col_vec);
    let col_worth = get_col_worth(&rounds_in_col, total_rows);
    total_worth = total_worth + col_worth;
  }
  println!("Total worth is {}", total_worth);
}

fn part_two() {
  let file_path = "data/d14.txt";
  let dish_matrix = get_matrix(&file_path);
  let num_cols = dish_matrix[0].len();
  let total_rows = dish_matrix.len();
  for _ in 0..1000000000 {
    for col_number in 0..num_cols {
      let col_vec = get_col(&dish_matrix, col_number);
      let rounds_in_col = get_rounds_in_col(&col_vec);
    }
  }

}


fn main() {
  // part_one();
  part_two();
}