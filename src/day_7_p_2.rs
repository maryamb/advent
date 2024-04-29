use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use crate::io::Error;


#[derive(Debug)]
#[derive(Eq, PartialEq)]
enum DeckType {
  FiveOfAKind,
  FourOfAKind,
  FullHouse,
  ThreeOfAKind,
  TwoPair,
  OnePair,
  HighCard,
}
use maplit::hashmap;
use DeckType::*;


impl Ord for DeckType {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (FiveOfAKind, FiveOfAKind) => Ordering::Equal,
      (FiveOfAKind, _) => Ordering::Greater,
      (_, FiveOfAKind) => Ordering::Less,

      (FourOfAKind, FourOfAKind) => Ordering::Equal,
      (FourOfAKind, _) => Ordering::Greater,
      (_, FourOfAKind) => Ordering::Less,

      (FullHouse, FullHouse) => Ordering::Equal,
      (FullHouse, _) => Ordering::Greater,
      (_, FullHouse) => Ordering::Less,

      (ThreeOfAKind, ThreeOfAKind) => Ordering::Equal,
      (ThreeOfAKind, _) => Ordering::Greater,
      (_, ThreeOfAKind) => Ordering::Less,

      (TwoPair, TwoPair) => Ordering::Equal,
      (TwoPair, _) => Ordering::Greater,
      (_, TwoPair) => Ordering::Less,

      (OnePair, OnePair) => Ordering::Equal,
      (OnePair, _) => Ordering::Greater,
      (_, OnePair) => Ordering::Less,

      _ => Ordering::Equal,
    }
  }
}

impl PartialOrd for DeckType {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}


#[derive(Debug)]
#[derive(Eq, PartialEq)]
struct Hand {
  cards: String,
  deck_type: DeckType,
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    let type_cmp = self.deck_type.cmp(&other.deck_type);
    if type_cmp == Ordering::Equal {
      return self.cards.cmp(&other.cards);
    }
    type_cmp
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}


fn get_type(deck: &Vec<usize>) -> DeckType {
  if deck.iter().any(|&d| d == 5_usize) { return DeckType::FiveOfAKind; }
  else if deck.iter().any(|&d| d == 4_usize) { return DeckType::FourOfAKind; }
  else if deck.iter().any(|&d| d == 3_usize) && deck.iter().any(|&d| d == 2_usize) { return DeckType::FullHouse; }
  else if deck.iter().any(|&d| d == 3_usize) { return DeckType::ThreeOfAKind; }
  else if deck.iter().filter(|&d| *d == 2_usize).count() == 2_usize { return DeckType::TwoPair; }
  else if deck.iter().any(|&d| d == 2_usize) { return DeckType::OnePair; }
  DeckType::HighCard
}

fn deck_to_hand(deck: &str) -> Hand {
  let mut cards = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
    .into_iter()
    .map(|c| deck.chars().filter(|&a| a == c).count())
    .collect::<Vec<usize>>();
  let j_count = deck.chars().filter(|&c| c == 'J').count();
  let cards_argmax = cards.iter().enumerate().max_by_key(|(_, &v)| v).map(|(idx, _)| idx);
  cards[cards_argmax.expect("")] += j_count;
  let deck_type = get_type(&cards);
  let transform_map: HashMap<char, char> = hashmap! {
    'A' => 'm',
    'K' => 'l',
    'Q' => 'k',
    'T' => 'j',
    '9' => 'i',
    '8' => 'h',
    '7' => 'g',
    '6' => 'f',
    '5' => 'e',
    '4' => 'd',
    '3' => 'c',
    '2' => 'b',
    'J' => 'a',
  }; 
  Hand{cards: deck.chars().into_iter().map(|c| transform_map[&c]).collect(), deck_type}
}


fn read_hands(file_path: &str) -> Result<Vec<(Hand, u64)>, Error> {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);
  let mut hand_values: Vec<(Hand, u64)> = Vec::new();

  for line in reader.lines() {
    let line = line?;
    let mut kv = line.split(' ');
    hand_values.push(
      (deck_to_hand(kv.next().expect("")), kv.next().expect("").parse().unwrap()));
  }
  hand_values.sort();

  Ok(hand_values)
}


fn part_two(file_path: &str) {
  let hand_values = read_hands(file_path).expect("");
  let weighted_sum = hand_values
    .into_iter()
    .enumerate()
    .fold(0_u64, |acc, idx_hand_val| acc + (idx_hand_val.0 as u64 + 1) * idx_hand_val.1.1);
  println!("weigthed_sum: {:?}", weighted_sum);
}


fn main() {
  let file_path = "data/d7.txt";
  println!("{:?}", file_path);
  part_two(&file_path);
}