use std::{io::BufRead, collections::HashMap};

pub fn what_is_the_sum_of_all_of_the_calibration_values(input: &mut dyn BufRead) -> u32 {
    return read_input(input).iter()
        .map(|s| recover_calibration_value(s))
        .sum();
}

pub fn what_is_the_real_sum_of_all_of_the_calibration_values(input: &mut dyn BufRead) -> u32 {
    return read_input(input).iter()
        .map(|s| recover_real_calibration_value(s))
        .sum();
}

fn read_input(input: &mut dyn BufRead) -> Vec<String> {
    return input.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
}

fn recover_calibration_value(line: &String) -> u32 {
    let mut iterator = line.chars().into_iter()
        .filter(|&c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap());

    let first = iterator.next().unwrap();
    let last = iterator.last().unwrap_or(first);
    
    return first * 10 + last;
}

fn recover_real_calibration_value(line: &String) -> u32 {
    let digits_spelled_out_with_letters: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]);

    let mut unified_line: String = line.clone();

    let first_spelled_as_letter = digits_spelled_out_with_letters.iter()
        .map(|(&k, _)| (unified_line.find(&k), k))
        .filter(|(pos, _)| pos.is_some())
        .map(|(pos, k)| (pos.unwrap(), k))
        .min_by_key(|(pos, _)| *pos);

    let first_digit_pos = unified_line.chars().into_iter().enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(pos, _)| pos)
        .next();

    if let Some(fs) = first_spelled_as_letter {
        if first_digit_pos.filter(|&fd| fd <= fs.0).is_none() {
            unified_line = unified_line.replacen(fs.1, &digits_spelled_out_with_letters.get(fs.1).unwrap().to_string(), 1);
        }
    }

    let last_spelled_as_letter = digits_spelled_out_with_letters.iter()
        .map(|(&k, _)| (unified_line.rfind(&k), k))
        .filter(|(pos, _)| pos.is_some())
        .map(|(pos, k)| (pos.unwrap(), k))
        .max_by_key(|(pos, _)| *pos);

    let last_digit_pos = unified_line.chars().into_iter().enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(pos, _ )| pos)
        .last();

    if let Some(ls) = last_spelled_as_letter {
        if last_digit_pos.filter(|&ld| ld >= ls.0).is_none() {
            unified_line.replace_range(ls.0..ls.0+ls.1.len()-1, &digits_spelled_out_with_letters.get(ls.1).unwrap().to_string());
        }
    }

    let mut iterator = unified_line.chars().into_iter()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap());

    let first = iterator.next().unwrap();
    let last = iterator.last().unwrap_or(first);

    return first * 10 + last;
}


#[cfg(test)]
mod tests {
    use crate::day01::{what_is_the_sum_of_all_of_the_calibration_values, what_is_the_real_sum_of_all_of_the_calibration_values};
    use std::{fs::File, io::BufReader};

    #[test]
    fn sample_part1_input() {
        let mut f = BufReader::new(File::open("./src/day01/sample-part1.input").unwrap());
        assert_eq!(what_is_the_sum_of_all_of_the_calibration_values(&mut f), 142);
    }

    #[test]
    fn part1_input() {
        let mut f = BufReader::new(File::open("./src/day01/my.input").unwrap());
        assert_eq!(what_is_the_sum_of_all_of_the_calibration_values(&mut f), 54573);
    }

    #[test]
    fn part2_samples() {
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "two1nine".as_bytes()), 29);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "eightwothree".as_bytes()), 83);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "abcone2threexyz".as_bytes()), 13);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "xtwone3four".as_bytes()), 24);

        let mut f = BufReader::new(File::open("./src/day01/sample-part2.input").unwrap());
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut f), 281);
    }

    #[test]
    fn part2_extra_samples() {
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "fourzvmlt3sggpjzssljc8twoeighttwo".as_bytes()), 42);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "oneight".as_bytes()), 11);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "1oneight".as_bytes()), 18);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "oneight8".as_bytes()), 18);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "oneight9".as_bytes()), 19);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "3oneight9".as_bytes()), 39);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "oneight39".as_bytes()), 19);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "39oneight".as_bytes()), 38);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "oneightwo".as_bytes()), 12);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "oneightwo3".as_bytes()), 13);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "3oneightwo".as_bytes()), 32);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "fiveeight2zxjpzffvdsevenjhjvjfiveone".as_bytes()), 51);
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut "1eight5eight".as_bytes()), 18);
    }

    #[test]
    fn part2_input() {
        let mut f = BufReader::new(File::open("./src/day01/my.input").unwrap());
        assert_eq!(what_is_the_real_sum_of_all_of_the_calibration_values(&mut f), 54591);
    }
}
