use std::io::BufRead;

use rayon::prelude::*;

use crate::read_input;

pub fn what_is_the_lowest_location_number_that_corresponds_to_any_of_the_initial_seed_numbers(input: &mut dyn BufRead) -> u64 {
    let lines = read_input(input);
    let mut parts = lines.split(|line| line == "");
    let seeds = parse_seeds(parts.next().unwrap());
    let maps = parts.map(|part| parse_map(part)).collect::<Vec<Map>>();

    return seeds.iter().map(|s| get_location(*s, &maps))
        .min()
        .unwrap();
}

pub fn what_is_the_lowest_location_number_that_corresponds_to_any_of_the_initial_seed_ranges(input: &mut dyn BufRead) -> u64 {
    let lines = read_input(input);
    let mut parts = lines.split(|line| line == "");
    let seeds = parse_seeds_ranges(parts.next().unwrap());
    let maps = parts.map(|part| parse_map(part)).collect::<Vec<Map>>();

    return seeds.par_iter()
        .map(|s| get_location(*s, &maps))
        .min()
        .unwrap();
}

fn parse_seeds(lines: &[String]) -> Vec<u64> {
    return lines.get(0).unwrap()
        .split_once("seeds: ").unwrap().1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
}

fn parse_seeds_ranges(lines: &[String]) -> Vec<u64> {
    let ranges = lines.get(0).unwrap()
        .split_once("seeds: ").unwrap().1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<(u64, u64)>>();
    
    return ranges.par_iter()
        .flat_map(|(start, length)| (*start..start+length))
        .collect();
}

fn parse_map(lines: &[String]) -> Map {
    let map_lines = lines.iter()
        .skip(1)
        .map(|line| parse_map_line(line))
        .collect();

    return Map {
        lines: map_lines
    }
}

fn parse_map_line(line: &String) -> Line {
    let numbers = line.split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let destination_range_start = *numbers.get(0).unwrap();
    let source_range_start = *numbers.get(1).unwrap();
    let range_length = *numbers.get(2).unwrap();

    return Line {
        source_range_start: source_range_start,
        source_range_end: source_range_start + range_length,
        mapping: destination_range_start as i64 - source_range_start as i64
    }
}

fn get_location(seed: u64, maps: &Vec<Map>) -> u64 {
    let mut index = 0;
    let mut value = seed;
    loop {
        if let Some(map) = maps.get(index) {
            index += 1;
            value = map.map(value);

            if index == 7 {
                return value;
            }
        }
        else {
            panic!("Map {} not found", index);
        }
    }
}

struct Map {
    lines: Vec<Line>
}

struct Line {
    source_range_start: u64,
    source_range_end: u64,
    mapping: i64
}

impl Map {
    fn map(&self, value: u64) -> u64 {
        return self.lines.iter()
            .find(|line| line.matches(value))
            .map_or(value, |line| line.map(value));
    }
}

impl Line {
    fn matches(&self, value: u64) -> bool {
        return self.source_range_start <= value && value < self.source_range_end;
    }

    fn map(&self, value: u64) -> u64 {
        return (value as i64 + self.mapping) as u64;
    }
}

#[cfg(test)]
mod tests {
    use std::{io::BufReader, fs::File};

    use crate::day05::{what_is_the_lowest_location_number_that_corresponds_to_any_of_the_initial_seed_numbers, what_is_the_lowest_location_number_that_corresponds_to_any_of_the_initial_seed_ranges};

    #[test]
    fn sample_part1_input() {
        let mut f = BufReader::new(File::open("./src/day05/sample.input").unwrap());
        assert_eq!(what_is_the_lowest_location_number_that_corresponds_to_any_of_the_initial_seed_numbers(&mut f), 35);
    }

    #[test]
    fn part1_input() {
        let mut f = BufReader::new(File::open("./src/day05/my.input").unwrap());
        assert_eq!(what_is_the_lowest_location_number_that_corresponds_to_any_of_the_initial_seed_numbers(&mut f), 173706076);
    }

    #[test]
    fn sample_part2_input() {
        let mut f = BufReader::new(File::open("./src/day05/sample.input").unwrap());
        assert_eq!(what_is_the_lowest_location_number_that_corresponds_to_any_of_the_initial_seed_ranges(&mut f), 46);
    }

    #[test]
    fn part2_input() {
        let mut f = BufReader::new(File::open("./src/day05/my.input").unwrap());
        assert_eq!(what_is_the_lowest_location_number_that_corresponds_to_any_of_the_initial_seed_ranges(&mut f), 11611182);
    }
}