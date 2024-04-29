use std::fmt::Error;
// use core::prelude;
use std::fs::File;
use std::io::{self, BufRead};
use std::default::Default;

// use regex::Match;

// use std::iter::zip;
// use crate::io::Error;
use std::collections::HashMap;

#[derive(Debug)]
enum Predicate {
  X,
  M,
  A,
  S
}

impl Predicate {
    fn new(predicate: char) -> Option<Predicate> {
      match predicate {
          'x' => Some(Predicate::X),
          'm' => Some(Predicate::M),
          'a' => Some(Predicate::A),
          's' => Some(Predicate::S),
          _  => None,
      }
    }
}

#[derive(Debug)]
enum Operation {
  Greater,
  Less,
}

impl Operation {
    fn new(operation: char) -> Option<Operation> {
      match operation {
          '<' => Some(Operation::Less),
          '>' => Some(Operation::Greater),
          _ => None,
      }
    }
}

#[derive(Debug)]
enum Admission {
  Accept,
  Reject,
}

impl Admission {
    fn new(admission: char) -> Option<Admission> {
      match admission {
        'R' => Some(Admission::Reject),
        'A' => Some(Admission::Accept),
        _ => None,
      }
    }
}

#[derive(Default)]
#[derive(Debug)]
struct Rule {
  predicate: Option<Predicate>,
  operation: Option<Operation>,
  fixed_point: Option<u32>,
  next_key: Option<String>,
  admission: Option<Admission>,
}

impl Rule {
    fn new(rule: &str) -> Rule {
      if let Some(colon_index) = rule.find(':') {
        let op_index = rule.find('>').or(rule.find('<')).unwrap();
        let predicate_index: usize = op_index - 1;
        let admission = Admission::new(rule.chars().nth(colon_index + 1).unwrap());
        Rule {
          predicate: Predicate::new(rule.chars().nth(predicate_index).unwrap()),
          operation: Operation::new(rule.chars().nth(op_index).unwrap()),
          fixed_point: Some(rule[op_index + 1 ..colon_index].parse().expect("there is always a number")),
          next_key: match admission {
              Some(_) => None,
              _ => Some(rule[colon_index + 1..].to_owned()),
          },
          admission: admission,
        } 
      } else {
        let admission = Admission::new(rule.chars().nth(0).unwrap());
        Rule {
          predicate: None,
          operation: None, 
          fixed_point: None,
          next_key: match admission {
              Some(_) => None,
              _ => Some(rule.to_owned()),
          },
          admission: admission,
        }
      }
    }
}

fn process_rules_line(line: &str) -> (String, Vec<Rule>) {
  let start_index = line.find('{').expect("Line has {");
  // Extract the substring before '{'
  let rule_key = &line[..start_index].trim();
  let end_index = line.find('}').expect("Line contains }");
  // Extract the substring inside the curly braces
  let rule_vec: Vec<Rule> = line[start_index + 1..end_index].split(',').into_iter().map(|rule_str| Rule::new(rule_str)).collect();

  (String::from(rule_key.to_owned()), rule_vec)
}

#[derive(Debug)]
struct Part {
  x: u32,
  m: u32,
  a: u32,
  s: u32,
  rating: u32,
}

impl Part {
    fn new(part: &str) -> Part {
      let xmas_vec: Vec<u32> = part[1..part.len() - 1].split(',').filter_map(|sub_part| {
        let mut split = sub_part.split('=');
        let _key = split.next()?; // Ignore the key part
        split.next()?.parse::<u32>().ok() // Parse the value part into i32
    }).collect();
    Part { x: xmas_vec[0], m: xmas_vec[1], a: xmas_vec[2], s: xmas_vec[3], rating: xmas_vec.iter().fold(0, |acc, el| { acc + el }) }
    }
}

fn does_part_match_rule(part: &Part, rule: &Rule) -> bool {
  match &rule.predicate {
      None => {true},
      Some(predicate) => {
        let part_predicate = match predicate {
        Predicate::X => part.x,
        Predicate::M => part.m,
        Predicate::A => part.a,
        Predicate::S => part.s,
      };
      match &rule.operation {
        Some(Operation::Less) => { part_predicate < rule.fixed_point.unwrap() },
        Some(Operation::Greater) => { part_predicate > rule.fixed_point.unwrap() },
        _ => { true }, // This never happens
      }
    }
  }
}

fn get_next_action<'a>(part: &'a Part, rules: &'a Vec<Rule>) -> (Option<&'a String>, Option<&'a Admission>) {
  let matching_rule = rules.iter().find(|&rule| { does_part_match_rule(part, rule) }).unwrap();
  (matching_rule.next_key.as_ref(), matching_rule.admission.as_ref())
}

type RulesMap = HashMap<String, Vec<Rule>>;
type RulesAndParts = (RulesMap, Vec<Part>);

fn read_rules_map_and_parts(file_path: &str) -> Result<RulesAndParts, Error> {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

  let mut rules_map = RulesMap::new();
  let mut parts: Vec<Part> = Vec::new();

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);
  reader.lines().into_iter().for_each(|line| {
    let line = line.unwrap();
    if line.starts_with('{') {
      parts.push(Part::new(&line));
    } else if line.ends_with('}') {
        let (k, v) = process_rules_line(&line);
        rules_map.insert(k, v);
    }
  });
  Ok((rules_map, parts))
}

fn follow_rules_to_admission(rules_map: &RulesMap, part: &Part) -> u32 {
  let mut next_key = "in";
  loop {
    let rules_part_match = get_next_action(part, &rules_map[next_key]);
    match rules_part_match.1 {
        Some(admission) => {
          match admission {
              Admission::Accept => { return part.rating; },
              Admission::Reject => { return 0; },
          }
        },
        None => {
          next_key = rules_part_match.0.unwrap();
        } 
    }
  }
}

fn get_ratings_for_admitted_parts(rules_map: &RulesMap, parts: &Vec<Part>) -> u32 {
  parts.iter().fold(0, |acc, part| {
    acc + follow_rules_to_admission(rules_map, part)
  })
}

#[allow(dead_code)]
fn some_tests() {
  let rule_1 = "hdj{m>838:A,pv}";
  let rule_2 = "gd{a>3333:R,R}";
  let rule_line_3 = "qqz{s>2770:qs,m<1801:hdj,R}";
  let (rule_key, rules) = process_rules_line(&rule_line_3);
  let part_1 = Part::new("{x=68,m=904,a=914,s=2699}");

  println!("{:#?}", part_1);
  println!("{:#?}", does_part_match_rule(&part_1, &rules[2]));
}

fn part_one() {
  let rules_and_parts = read_rules_map_and_parts("data/d19.txt").expect("File exists.");
  let rules_map = rules_and_parts.0;
  let parts = rules_and_parts.1;
  println!("{:#?}", get_ratings_for_admitted_parts(&rules_map, &parts));
}

fn main() {
  part_one();
  // part_two();
}