pub fn exec(input: &str) -> String {
  process(input).to_string()
}

fn split_vec<'a>(v: &Vec<&'a str>, item: &str) -> (Vec<&'a str>, Vec<&'a str>) {
  let mut vec1: Vec<&str> = vec![];
  let mut break_index: usize = 0;
  for (index, value) in v.iter().enumerate() {
    if value == &"" {
      continue;
    }
    if value == &item {
      break_index = index;
      break;
    }
    vec1.push(value);
  }
  (vec1, v[break_index + 1..].to_vec())
}

struct Almanac {
  seeds: Vec<usize>,
  to_soil: Vec<Map>,
  to_fertilizer: Vec<Map>,
  to_water: Vec<Map>,
  to_light: Vec<Map>,
  to_temperature: Vec<Map>,
  to_humidity: Vec<Map>,
  to_location: Vec<Map>,
}

impl From<&str> for Almanac {
  fn from(value: &str) -> Self {
    let lines = value.lines().collect::<Vec<&str>>();

    let (seeds_vecline, remaining) = split_vec(&lines, "seed-to-soil map:");
    let (_, seeds_numbers) = seeds_vecline[0].trim().split_once(':').unwrap();
    let seeds = seeds_numbers
      .trim()
      .split(' ')
      .map(|n| n.trim().parse::<usize>().unwrap())
      .collect::<Vec<usize>>();

    let (veclines, remaining) = split_vec(&remaining, "soil-to-fertilizer map:");
    let to_soil: Vec<Map> = veclines.iter().map(|&item| Map::from(item)).collect();

    let (veclines, remaining) = split_vec(&remaining, "fertilizer-to-water map:");
    let to_fertilizer: Vec<Map> = veclines.iter().map(|&item| Map::from(item)).collect();

    let (veclines, remaining) = split_vec(&remaining, "water-to-light map:");
    let to_water: Vec<Map> = veclines.iter().map(|&item| Map::from(item)).collect();

    let (veclines, remaining) = split_vec(&remaining, "light-to-temperature map:");
    let to_light: Vec<Map> = veclines.iter().map(|&item| Map::from(item)).collect();

    let (veclines, remaining) = split_vec(&remaining, "temperature-to-humidity map:");
    let to_temperature: Vec<Map> = veclines.iter().map(|&item| Map::from(item)).collect();

    let (veclines, remaining) = split_vec(&remaining, "humidity-to-location map:");
    let to_humidity: Vec<Map> = veclines.iter().map(|&item| Map::from(item)).collect();

    let to_location: Vec<Map> = remaining.iter().map(|&item| Map::from(item)).collect();

    Self {
      seeds,
      to_soil,
      to_fertilizer,
      to_water,
      to_light,
      to_temperature,
      to_humidity,
      to_location,
    }
  }
}

struct Map {
  source_start: usize,
  dest_start: usize,
  len: usize,
}

impl From<&str> for Map {
  fn from(value: &str) -> Self {
    let values = value.trim().split(' ').collect::<Vec<&str>>();
    Self {
      source_start: values[1].trim().parse::<usize>().unwrap(),
      dest_start: values[0].trim().parse::<usize>().unwrap(),
      len: values[2].trim().parse::<usize>().unwrap(),
    }
  }
}

impl Map {
  pub fn on_source(&self, n: usize) -> bool {
    self.source_start <= n && n < (self.source_start + self.len)
  }

  pub fn new_position(&self, n: usize) -> usize {
    if self.dest_start < self.source_start {
      n.saturating_sub(self.source_start - self.dest_start)
    } else {
      n + (self.dest_start - self.source_start)
    }
  }
}

fn cross_position_and_maps(position: &usize, maps: &Vec<Map>) -> usize {
  for map in maps.iter() {
    if map.on_source(*position) {
      return map.new_position(*position);
    }
  }
  position.clone()
}

fn cross_pos_and_maps(positions: &Vec<usize>, maps: &Vec<Map>) -> Vec<usize> {
  positions
    .iter()
    .map(|position| cross_position_and_maps(position, maps))
    .collect()
}

struct FinalPosition(Vec<usize>);

impl From<Almanac> for FinalPosition {
  fn from(value: Almanac) -> Self {
    let after_soil = cross_pos_and_maps(&value.seeds, &value.to_soil);
    let after_ferti = cross_pos_and_maps(&after_soil, &value.to_fertilizer);
    let after_water = cross_pos_and_maps(&after_ferti, &value.to_water);
    let after_light = cross_pos_and_maps(&after_water, &value.to_light);
    let after_tempe = cross_pos_and_maps(&after_light, &value.to_temperature);
    let after_humid = cross_pos_and_maps(&after_tempe, &value.to_humidity);
    let after_locat = cross_pos_and_maps(&after_humid, &value.to_location);
    FinalPosition(after_locat)
  }
}

fn process(input: &str) -> usize {
  *(FinalPosition::from(Almanac::from(input)).0)
    .iter()
    .min()
    .unwrap()
}

#[cfg(test)]
mod testsday05part01 {
  use super::*;

  #[test]
  pub fn example() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(process(input), 35)
  }
}
