use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::fmt;

// use crate::io::Error;
// use std::collections::HashMap;


type Matrix = Vec<Vec<char>>;


fn get_col(dish_matrix: &Vec<String>, col: usize) -> Vec<char> {
  dish_matrix.iter().filter_map(|s| s.chars().nth(col)).collect()
}

fn get_matrix(file_path: &str) -> Matrix {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
      Err(why) => panic!("couldn't open {}: {}", file_path, why),
      Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);
  let lines = reader.lines();
  let mut tmp_matrix = Vec::new();
  for line in lines {
    tmp_matrix.push(line.expect("Line is non empty"));
  }

  let num_cols = tmp_matrix[0].len();

  let mut dish_matrix: Matrix = Vec::new();
  for col_n in 0..num_cols {
    dish_matrix.push(get_col(&tmp_matrix, col_n));
  }
  dish_matrix
}


fn get_square_rock_indices(col_vec: &Vec<char>) -> Vec<usize> {
  col_vec
      .iter()
      .enumerate()
      .filter_map(|(index, &character)| if character == '#' { Some(index) } else { None } )
      .collect()
}

fn tilt_col_north(col_vec: &Vec<char>) -> Vec<char> {
  let occurrences = |s, e| col_vec[s..e]
        .iter()
        .filter(|&&c| c == 'O')
        .count();
  let squares = get_square_rock_indices(col_vec);
  let starts = [vec![0usize], squares.iter().map(|a| a + 1).collect()].concat();
  let ends = [squares.clone(), vec![col_vec.len()]].concat();
  let intervals = zip(starts, ends);
  let start_count_vec: Vec<(usize, usize)> = intervals.filter_map(|(s, e)| Some((s, occurrences(s, e)))).collect();
  let mut new_col: Vec<char> = vec!['.'; col_vec.len()];
  for square_ind in squares {
    new_col[square_ind] = '#';
  }
  for sc in start_count_vec {
    for ind in sc.0..sc.0 + sc.1 {
      new_col[ind] = 'O';
    }
  }
  new_col
}

fn tilt_north(dish_matrix: &Matrix) -> Matrix {
  let mut new_matrix = Matrix::new();
  dish_matrix.iter().for_each(|col| new_matrix.push(tilt_col_north(col)));
  new_matrix
}

fn rotate_90_clock(matrix: &Matrix) -> Matrix {
  if matrix.is_empty() || matrix[0].is_empty() {
    return matrix.clone();
  }
  let rows = matrix.len();
  let cols = matrix[0].len();

  let mut result = vec![vec!['.'; rows]; cols];

  for i in 0..rows {
      for j in 0..cols {
          result[j][rows - 1 - i] = matrix[i][j];
      }
  }

  result
}



fn interval_worth(start_ind: usize, num: usize, total_rows: usize) -> i32 {
  let b = total_rows - start_ind;
  let a = b - num + 1;
  ((a + b) * num / 2) as i32
}

fn get_col_worth(rounds_in_col: &Vec<(usize, usize)>, total_rows: usize) -> i32 {
  rounds_in_col.iter().fold(0, |acc, (s, n)| acc + interval_worth(*s, *n, total_rows))
}



fn part_two() {
  let file_path = "data/d14_ex.txt";
  let dish_matrix = get_matrix(&file_path);
  println!("dish_matrix is {:?}", dish_matrix);
}


fn main() {
  // part_one();
  part_two();
}