
fn get_greater_dist_count(time: u64, recorded_score: u64) -> u64 {
  (1..time)
    .into_iter()
    .fold(0, |acc, t| if t * (time - t) > recorded_score { acc + 1 } else {acc} )
}

fn part_one() {
  let times = vec![48_u64,     87,     69,     81];
  let recorded_scores  = vec![255_u64,   1288,   1117,   1623];
  let scores_prod = 
    times.into_iter().zip(recorded_scores.into_iter())
      .fold(1, |prod, (t, s)| prod * get_greater_dist_count(t, s));
  println!("Max scores prod is {scores_prod}");
}

fn part_two() {
  let time = 48876981;
  let recorded_score  = 255128811171623;
  let greater_dist_count = get_greater_dist_count(time, recorded_score);
  println!("Max scores prod is {}", greater_dist_count);
}


fn main() {
  part_two();
}