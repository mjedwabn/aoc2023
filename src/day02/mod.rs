use std::{io::BufRead, collections::HashMap};
use regex::Regex;

use crate::read_input;

pub fn what_is_the_sum_of_the_ids_of_possible_games(input: &mut dyn BufRead) -> u32 {
    return read_input(input).iter()
        .map(|s| parse_game(s))
        .map(|(id, subsets)| (id, is_game_possible(subsets)))
        .filter(|(_, possible)| *possible)
        .map(|(id, _)| id)
        .sum();
}

pub fn what_is_the_sum_of_the_power_of_sets(input: &mut dyn BufRead) -> u32 {
    return read_input(input).iter()
        .map(|s| parse_game(s))
        .map(|(_, subsets)| get_power(subsets))
        .sum();
}

fn parse_game(line: &String) -> (u32, Vec<HashMap<&str, u32>>) {
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let captures = re.captures(line).unwrap();
    let game_id = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let subsets = captures.get(2).unwrap().as_str().split(';')
        .map(|s| parse_subset(s));
    return (game_id, subsets.collect());
}

fn parse_subset(raw_subset: &str) -> HashMap<&str, u32> {
    return raw_subset.split(',')
        .map(|raw_cubes| parse_cubes(raw_cubes))
        .into_iter().collect();
}

fn parse_cubes(raw_cubes: &str) -> (&str, u32) {
    let re = Regex::new(r"(\d+) (.*)").unwrap();
    let captures = re.captures(raw_cubes).unwrap();
    return (captures.get(2).unwrap().as_str(), captures.get(1).unwrap().as_str().parse::<u32>().unwrap())
}

fn is_game_possible(cube_subsets: Vec<HashMap<&str, u32>>) -> bool {
    return cube_subsets.iter()
        .all(|subset| is_subset_possible(subset));
}

fn is_subset_possible(subset: &HashMap<&str, u32>) -> bool {
    return subset.get("red").map_or(true, |&x| x <= 12)
        && subset.get("green").map_or(true, |&x| x <= 13)
        && subset.get("blue").map_or(true, |&x| x <= 14);
}

fn get_power(cube_subsets: Vec<HashMap<&str, u32>>) -> u32 {
    let rgb: Vec<(u32, u32, u32)> = cube_subsets.iter()
        .map(|c| (
            c.get("red").map(|v| *v).unwrap_or(0), 
            c.get("green").map(|v| *v).unwrap_or(0), 
            c.get("blue").map(|v| *v).unwrap_or(0)
        ))
        .collect();

    let max_r = rgb.iter().max_by_key(|c| c.0).map(|c| c.0).unwrap_or(0);
    let max_g = rgb.iter().max_by_key(|c| c.1).map(|c| c.1).unwrap_or(0);
    let max_b = rgb.iter().max_by_key(|c| c.2).map(|c| c.2).unwrap_or(0);

    return max_r * max_g * max_b;
}

mod tests {
    use crate::day02::{what_is_the_sum_of_the_ids_of_possible_games, what_is_the_sum_of_the_power_of_sets};
    use std::{fs::File, io::BufReader};

    #[test]
    fn sample_part1_input() {
        let mut f = BufReader::new(File::open("./src/day02/sample.input").unwrap());
        assert_eq!(what_is_the_sum_of_the_ids_of_possible_games(&mut f), 8);
    }

    #[test]
    fn part1_input() {
        let mut f = BufReader::new(File::open("./src/day02/my.input").unwrap());
        assert_eq!(what_is_the_sum_of_the_ids_of_possible_games(&mut f), 1734);
    }

    #[test]
    fn sample_part2_input() {
        let mut f = BufReader::new(File::open("./src/day02/sample.input").unwrap());
        assert_eq!(what_is_the_sum_of_the_power_of_sets(&mut f), 2286);
    }

    #[test]
    fn part2_input() {
        let mut f = BufReader::new(File::open("./src/day02/my.input").unwrap());
        assert_eq!(what_is_the_sum_of_the_power_of_sets(&mut f), 70387);
    }
}
