use std::io::BufRead;

use crate::read_input;

pub fn what_is_the_sum_of_all_of_the_part_numbers_in_the_engine_schematic(input: &mut dyn BufRead) -> u32 {
    let schematic = parse_schematic(input);
    
    return schematic.get_numbers().iter()
        .filter(|&n| schematic.is_part_number(n))
        .map(|pn| schematic.to_number(pn))
        .sum();
}

pub fn what_is_the_sum_of_all_of_the_gear_ratios_in_the_engine_schematic(input: &mut dyn BufRead) -> u32 {
    let schematic = parse_schematic(input);

    return schematic.find('*').iter()
        .map(|c| schematic.get_adjacent_numbers(c))
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers.iter().map(|n| schematic.to_number(n)).fold(1, |acc, elem| acc * elem))
        .sum();
}

fn parse_schematic(input: &mut dyn BufRead) -> Schematic {
    let grid = read_input(input).iter()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect();
    return Schematic {
        grid
    }
}

struct Schematic {
    grid: Vec<Vec<char>>
}

impl Schematic {    
    fn get_numbers(&self) -> Vec<Vec<(usize, usize)>> {
        return self.grid.iter()
            .zip(0..self.grid.len())
            .flat_map(|(row, y)| self.find_numbers(y, row))
            .collect();
    }

    fn find_numbers(&self, y: usize, row: &Vec<char>) -> Vec<Vec<(usize, usize)>> {
        let mut numbers: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut it = row.iter().zip(0..row.len());

        loop {
            let number: Vec<(usize, usize)> = it.by_ref()
                .skip_while(|x| !x.0.is_digit(10))
                .take_while(|x| x.0.is_digit(10))
                .map(|x| (x.1, y))
                .collect();

            if number.len() == 0 {
                break;
            }
            else {
                numbers.push(number);
            }
        }

        return numbers;
    }

    fn is_part_number(&self, number: &Vec<(usize, usize)>) -> bool {
        return number.iter().any(|c| self.is_adjacent_to_part(c));
    }

    fn is_adjacent_to_part(&self, coord: &(usize, usize)) -> bool {
        return self.get_adjacent_coords(coord).iter().any(|c| self.is_part(c));
    }

    fn is_part(&self, coord: &(usize, usize)) -> bool {
        let field = self.get(coord);
        return !field.is_digit(10) && *field != '.';
    }

    fn get(&self, coord: &(usize, usize)) -> &char {
        return self.grid.get(coord.1).unwrap().get(coord.0).unwrap();
    }

    fn get_adjacent_coords(&self, coord: &(usize, usize)) -> Vec<(usize, usize)> {
        let cc = (coord.0 as isize, coord.1 as isize);

        return vec![
            (cc.0 - 1, cc.1),
            (cc.0 - 1, cc.1 - 1),
            (cc.0, cc.1 - 1),
            (cc.0 - 1, cc.1 + 1),
            (cc.0 + 1, cc.1),
            (cc.0 + 1, cc.1 - 1),
            (cc.0, cc.1 + 1),
            (cc.0 + 1, cc.1 + 1),
        ].iter()
            .filter(|&c| self.in_grid(c))
            .map(|c| (c.0 as usize, c.1 as usize))
            .collect::<Vec<(usize, usize)>>();
    }

    fn in_grid(&self, coord: &(isize, isize)) -> bool {
        return coord.1 >= 0 && coord.1 < self.grid.len() as isize 
            && coord.0 >= 0 && coord.0 < self.grid.get(coord.1 as usize).unwrap().len() as isize;
    }

    fn to_number(&self, number: &Vec<(usize, usize)>) -> u32 {
        return number.iter()
            .map(|c| self.get(c).to_digit(10).unwrap())
            .fold(0, |acc, elem| acc * 10 + elem);
    }

    fn find(&self, symbol: char) -> Vec<(usize, usize)> {
        return self.coords().iter()
            .filter(|c| *self.get(c) == symbol)
            .map(|&c| c)
            .collect();
    }

    fn coords(&self) -> Vec<(usize, usize)> {
        return self.grid.iter()
            .zip(0..self.grid.len())
            .flat_map(|(row, y)| row.iter().zip(0..row.len()).map(move |(_, x)| (x, y)))
            .collect();
    }

    fn get_adjacent_numbers(&self, coord: &(usize, usize)) -> Vec<Vec<(usize, usize)>> {
        return self.get_numbers().into_iter()
            .filter(|n| self.is_adjacent_to_any(coord, &n))
            .collect();
    }

    fn is_adjacent_to_any(&self, coord: &(usize, usize), coords: &Vec<(usize, usize)>) -> bool {
        return self.get_adjacent_coords(&coord).iter()
            .any(|adj_c| coords.iter().any(|n| adj_c == n));
    }
}


#[cfg(test)]
mod tests {
    use std::{io::BufReader, fs::File};

    use crate::day03::{what_is_the_sum_of_all_of_the_part_numbers_in_the_engine_schematic, what_is_the_sum_of_all_of_the_gear_ratios_in_the_engine_schematic};

    #[test]
    fn sample_part1_input() {
        let mut f = BufReader::new(File::open("./src/day03/sample.input").unwrap());
        assert_eq!(what_is_the_sum_of_all_of_the_part_numbers_in_the_engine_schematic(&mut f), 4361);
    }

    #[test]
    fn part1_input() {
        let mut f = BufReader::new(File::open("./src/day03/my.input").unwrap());
        assert_eq!(what_is_the_sum_of_all_of_the_part_numbers_in_the_engine_schematic(&mut f), 514969);
    }

    #[test]
    fn sample_part2_input() {
        let mut f = BufReader::new(File::open("./src/day03/sample.input").unwrap());
        assert_eq!(what_is_the_sum_of_all_of_the_gear_ratios_in_the_engine_schematic(&mut f), 467835);
    }

    #[test]
    fn part2_input() {
        let mut f = BufReader::new(File::open("./src/day03/my.input").unwrap());
        assert_eq!(what_is_the_sum_of_all_of_the_gear_ratios_in_the_engine_schematic(&mut f), 78915902);
    }
}
