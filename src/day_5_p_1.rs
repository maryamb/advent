use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::Lines;
use std::collections::BTreeMap;
use crate::io::Error;
// use regex::Regex;


fn get_seeds(line: &str) -> Vec<u64> {
  line
    .split(':')
    .nth(1)
    .unwrap_or("")
    .split(' ')
    .filter_map(|s| s.parse().ok())
    .collect::<Vec<u64>>()
}


#[derive(Debug)]
struct SeedsAndMaps {
  seed_to_soil: BTreeMap::<u64, (u64, u64)>,
  soil_to_fertilizer: BTreeMap::<u64, (u64, u64)>,
  fertilizer_to_water: BTreeMap::<u64, (u64, u64)>,
  water_to_light: BTreeMap::<u64, (u64, u64)>,
  light_to_temperature: BTreeMap::<u64, (u64, u64)>,
  temperature_to_humidity: BTreeMap::<u64, (u64, u64)>,
  humidity_to_location: BTreeMap::<u64, (u64, u64)>,
  seeds: Vec<u64>,
}


impl SeedsAndMaps {
  fn new() -> SeedsAndMaps {
    SeedsAndMaps {
      seed_to_soil: BTreeMap::<u64, (u64, u64)>::new(),
      soil_to_fertilizer: BTreeMap::<u64, (u64, u64)>::new(),
      fertilizer_to_water: BTreeMap::<u64, (u64, u64)>::new(),
      water_to_light: BTreeMap::<u64, (u64, u64)>::new(),
      light_to_temperature: BTreeMap::<u64, (u64, u64)>::new(),
      temperature_to_humidity: BTreeMap::<u64, (u64, u64)>::new(),
      humidity_to_location: BTreeMap::<u64, (u64, u64)>::new(),
      seeds: vec![],
    }
  }

  fn seed_to_location(&self, seed: u64) -> u64 {
    [
      &self.seed_to_soil,
      &self.soil_to_fertilizer,
      &self.fertilizer_to_water,
      &self.water_to_light,
      &self.light_to_temperature,
      &self.temperature_to_humidity,
      &self.humidity_to_location
    ].into_iter().fold(seed, get_destination_index)
  }

  fn get_min_seeds_location(&self) -> u64 {
    self.seeds
      .iter()
      .cloned()
      .map(|seed| self.seed_to_location(seed))
      .min()
      .expect("It is not empty.")
  }
}


fn fill_maps(lines: Lines<BufReader<File>>) -> Result<SeedsAndMaps, Error> {
  let mut sam = SeedsAndMaps::new();
  let mut current_map = &mut sam.seed_to_soil;
  for line in lines {
    let line = line?;
    if line.starts_with("seeds") {
      sam.seeds = get_seeds(&line);
    } else if line.ends_with(':') {
      let map_name = line.split(' ').next().unwrap_or("");
      match map_name {
        "seed-to-soil" => current_map = &mut sam.seed_to_soil,
        "soil-to-fertilizer" => current_map = &mut sam.soil_to_fertilizer,
        "fertilizer-to-water" => current_map = &mut sam.fertilizer_to_water,
        "water-to-light" => current_map = &mut sam.water_to_light,
        "light-to-temperature" => current_map = &mut sam.light_to_temperature,
        "temperature-to-humidity" => current_map = &mut sam.temperature_to_humidity,
        "humidity-to-location" => current_map = &mut sam.humidity_to_location,
        _ => {},
      }
    } else if !line.is_empty() {
        let tmp_vec = line.split(' ').filter_map(|s| s.parse().ok()).collect::<Vec<u64>>();
        current_map.insert(tmp_vec[1], (tmp_vec[0], tmp_vec[2]));
      }
  }
  Ok(sam)
}


fn get_destination_index(src_index: u64, src_to_dest: &BTreeMap::<u64, (u64, u64)>) -> u64 {
  let largest_key = src_to_dest.range(..=src_index).next_back();
  match largest_key {
    Some(kv) => {
      if kv.0 + kv.1.1 >= src_index {
        src_index - kv.0 + kv.1.0
      } else {
        src_index
      }
    },
    None => src_index
  }
}


fn make_sam(file_path: &str) -> Result<SeedsAndMaps, Error> {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
        Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);
  let sam = fill_maps(reader.lines())?;

  Ok(sam)
}


fn main() {
  let sam = make_sam("data/d5_p1_ex.txt").expect("File exists.");
  println!("{}", sam.get_min_seeds_location());
}