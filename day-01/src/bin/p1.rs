use std::fs::read_to_string;

fn main() {
    let path = "src/input.txt";
    let input = read_to_string(path).unwrap();
    let output = solve(input.as_str());
    println!("Result: {}", output);
}

fn solve(input: &str) -> usize {
    let mut sum: usize = 0;
    for line in input.lines() {
        sum += get_calibration_value(line);
    }

    return sum;
}

fn get_calibration_value(line: &str) -> usize {
    let mut first_num: char = '\0';
    let mut last_num: char = '\0';
    for char in line.chars() {
        if char.is_ascii_digit() {
            if first_num == '\0' {
                first_num = char;
            }
            last_num = char;
        }
    }

    let result = format!("{}{}", first_num, last_num);
    return result.parse().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let example_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = solve(example_input);
        let expected = 142;
        assert_eq!(result, expected);
    }
}
