use std::ops::Range;

pub fn exec() -> () {
  let input = include_str!("input1.txt");
  let output = process(input);
  println!("Day 05 - Part 02 - {output}");
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

#[derive(Debug)]
struct Almanac {
  seeds: Vec<Range<usize>>,
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

    let seeds_vec = seeds_numbers.trim().split(' ').collect::<Vec<&str>>();
    let mut seeds: Vec<Range<usize>> = vec![];
    for index in (0..seeds_vec.len()).step_by(2) {
      let start = seeds_vec[index].parse::<usize>().unwrap();
      let len = seeds_vec[index + 1].parse::<usize>().unwrap();
      seeds.push(start..(start + len));
    }

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

#[derive(Debug)]
struct Map {
  source_start: usize,
  dest_start: usize,
  length: usize,
}

impl From<&str> for Map {
  fn from(value: &str) -> Self {
    let values = value.trim().split(' ').collect::<Vec<&str>>();
    Self::new(
      values[1].trim().parse::<usize>().unwrap(),
      values[0].trim().parse::<usize>().unwrap(),
      values[2].trim().parse::<usize>().unwrap(),
    )
  }
}

impl Map {
  pub const fn new(source_start: usize, dest_start: usize, length: usize) -> Self {
    Self {
      source_start,
      dest_start,
      length,
    }
  }
}

fn cross_ranges_and_maps(ranges: Vec<Range<usize>>, maps: &Vec<Map>) -> Vec<Range<usize>> {
  let mut arr = ranges.clone();
  let mut idx = 0;
  loop {
    if let None = arr.get_mut(idx) {
      break;
    }
    let current_range = arr[idx].clone();

    for map in maps.iter() {
      let range = map.source_start..(map.source_start + map.length);

      let current_start = current_range.start;
      let current_end = current_range.end - 1;

      let start_distance = current_start.saturating_sub(map.source_start);
      let end_distance = current_end.saturating_sub(map.source_start);

      if range.contains(&current_start) && range.contains(&current_end) {
        // Range Overlap every point in range
        arr[idx] = (map.dest_start + start_distance)..(map.dest_start + end_distance);
        break;
      } else if range.contains(&current_start) && !range.contains(&current_end) {
        // Range Overlap on start
        arr[idx] = (map.dest_start + start_distance)..(map.dest_start + map.length);
        let not_overlaped = (map.source_start + map.length)..(current_end + 1);
        arr.insert(idx + 1, not_overlaped);
        break;
      } else if !range.contains(&current_start) && range.contains(&current_end) {
        // Range Overlap on end
        arr[idx] = (map.dest_start)..(map.dest_start + end_distance);
        let not_overlaped = (current_start)..(map.source_start);
        arr.insert(idx + 1, not_overlaped);
        break;
      }
    }
    idx += 1;
  }
  arr
}

struct FinalPosition(Vec<Range<usize>>);

impl From<Almanac> for FinalPosition {
  fn from(value: Almanac) -> Self {
    let after_soil = cross_ranges_and_maps(value.seeds.clone(), &value.to_soil);
    let after_ferti = cross_ranges_and_maps(after_soil, &value.to_fertilizer);
    let after_water = cross_ranges_and_maps(after_ferti, &value.to_water);
    let after_light = cross_ranges_and_maps(after_water, &value.to_light);
    let after_tempe = cross_ranges_and_maps(after_light, &value.to_temperature);
    let after_humid = cross_ranges_and_maps(after_tempe, &value.to_humidity);
    let after_locat = cross_ranges_and_maps(after_humid, &value.to_location);
    FinalPosition(after_locat)
  }
}

fn process(input: &str) -> usize {
  let almanac = Almanac::from(input);
  let final_positions = FinalPosition::from(almanac).0;
  let min = final_positions
    .iter()
    .map(|range| range.start)
    .min()
    .unwrap();
  min
}

#[cfg(test)]
mod test_day05_part02_example {
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
    assert_eq!(process(input), 46);
  }
}
