use std::fs::read_to_string;

fn main() {
    let path = "src/input.txt";
    let input = read_to_string(path).unwrap();
    let output = solve(input.as_str());
    println!("Result: {}", output)
}

fn solve(input: &str) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        sum += get_advanced_calibrated_value(line);
    }

    return sum;
}

fn get_advanced_calibrated_value(line: &str) -> usize {
    let mut first_num: char = '\0';
    let mut last_num: char = '\0';
    for (i, mut char) in line.chars().enumerate() {
        let mut is_digit = false;

        if char.is_ascii_digit() {
            is_digit = true;
        }

        let rest_of_line = &line[i..];
        if contains_spelled_digits(rest_of_line) {
            is_digit = true;
            char = parse_spelled_digit(rest_of_line);
        }

        if !is_digit {
            continue;
        };

        if first_num == '\0' {
            first_num = char;
        }
        last_num = char;
    }

    let result = format!("{}{}", first_num, last_num);
    return result.parse().unwrap();
}

fn contains_spelled_digits(segment: &str) -> bool {
    if segment.len() >= 5 {
        let possible_digit = &segment[0..5];
        let digits = ["three", "seven", "eight"];
        if digits.contains(&possible_digit) {
            return true;
        }
    }

    if segment.len() >= 4 {
        let possible_digit = &segment[0..4];
        let digits = ["four", "five", "nine"];
        if digits.contains(&possible_digit) {
            return true;
        }
    }

    if segment.len() >= 3 {
        let possible_digit = &segment[0..3];
        let digits = ["one", "two", "six"];
        if digits.contains(&possible_digit) {
            return true;
        }
    }

    return false;
}

fn parse_spelled_digit(segment: &str) -> char {
    if segment.len() >= 5 {
        let possible_digit = &segment[0..5];
        match possible_digit {
            "three" => return '3',
            "seven" => return '7',
            "eight" => return '8',
            &_ => (),
        }
    }

    if segment.len() >= 4 {
        let possible_digit = &segment[0..4];
        match possible_digit {
            "four" => return '4',
            "five" => return '5',
            "nine" => return '9',
            &_ => (),
        }
    }

    if segment.len() >= 3 {
        let possible_digit = &segment[0..3];
        match possible_digit {
            "one" => return '1',
            "two" => return '2',
            "six" => return '6',
            &_ => (),
        }
    }

    panic!("segment does not start with a spelled digit")
}

#[cfg(test)]
mod p2 {
    use super::*;

    #[test]
    fn sample() {
        let example_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = solve(example_input);
        let expected = 281;
        assert_eq!(result, expected);
    }
}
