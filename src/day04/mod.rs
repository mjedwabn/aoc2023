use std::{io::BufRead, collections::HashMap};
use regex::Regex;

use crate::read_input;

pub fn how_many_points_are_cards_worth_in_total(input: &mut dyn BufRead) -> u32 {
    return read_input(input).iter()
        .map(|line| parse_card(line))
        .map(|card| card.how_many_points_is_worth())
        .sum();
}

pub fn how_many_total_scratchcards_do_you_end_up_with(input: &mut dyn BufRead) -> u32 {
    let cards: Vec<Card> = read_input(input).iter()
        .map(|line| parse_card(line))
        .collect();

    let mut scratchcards: HashMap<u32, u32> = HashMap::new();

    cards.iter().rev().for_each(|card| {
        let copies: u32 = (1..=card.how_many_numbers_match())
            .map(|i| scratchcards.get(&(i + card.number)).unwrap())
            .sum();

        scratchcards.insert(card.number, copies + 1);
    });

    return scratchcards.values().sum();
}

fn parse_card(line: &String) -> Card {
    let re = Regex::new(r"Card\s+(\d+):\s+(.*)\s+\|\s+(.*)").unwrap();
    let captures = re.captures(line).unwrap();
    let number = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let winning_numbers = captures.get(2).unwrap().as_str()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let your_numbers = captures.get(3).unwrap().as_str()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    return Card {
        number,
        winning_numbers,
        your_numbers
    };
}

struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>
}

impl Card {
    fn how_many_points_is_worth(&self) -> u32 {
        let n = self.how_many_numbers_match();
    
        if n > 0 {
            return (2 as u32).pow(n - 1);
        }
        else {
            return 0;
        }
    }

    fn how_many_numbers_match(&self) -> u32 {
        return self.your_numbers.iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .collect::<Vec<&u32>>()
            .len() as u32;
    }
}

#[cfg(test)]
mod tests {
    use std::{io::BufReader, fs::File};

    use crate::day04::{how_many_points_are_cards_worth_in_total, how_many_total_scratchcards_do_you_end_up_with};

    #[test]
    fn sample_part1_input() {
        let mut f = BufReader::new(File::open("./src/day04/sample.input").unwrap());
        assert_eq!(how_many_points_are_cards_worth_in_total(&mut f), 13);
    }

    #[test]
    fn part1_input() {
        let mut f = BufReader::new(File::open("./src/day04/my.input").unwrap());
        assert_eq!(how_many_points_are_cards_worth_in_total(&mut f), 32001);
    }

    #[test]
    fn sample_part2_input() {
        let mut f = BufReader::new(File::open("./src/day04/sample.input").unwrap());
        assert_eq!(how_many_total_scratchcards_do_you_end_up_with(&mut f), 30);
    }

    #[test]
    fn part2_input() {
        let mut f = BufReader::new(File::open("./src/day04/my.input").unwrap());
        assert_eq!(how_many_total_scratchcards_do_you_end_up_with(&mut f), 5037841);
    }
}