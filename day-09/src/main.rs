use std::fs::read_to_string;
use std::num::ParseIntError;

fn main() {
    let path = "src/input.txt";
    let input = read_to_string(path).unwrap();
    let output = solve_p1(input.as_str());
    println!("Problem One Result: {}", output);
    let output = solve_p2(input.as_str());
    println!("Problem Two Result: {}", output);
}

fn solve_p1(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.lines() {
        let numbers = string_to_int_vect(line);
        result += predict_next_value(&numbers);
    }
    return result;
}

fn string_to_int_vect(string: &str) -> Vec<i64> {
    let collect: Result<Vec<i64>, ParseIntError> = string
        .split_whitespace()
        .map(|str_num| str_num.parse::<i64>())
        .collect();
    let numbers = collect.unwrap();
    return numbers;
}

fn predict_next_value(numbers: &[i64]) -> i64 {
    let mut predicted_value: i64 = *numbers.last().unwrap();
    let mut differences = get_differences(numbers);

    while !differences.iter().all(|&x| x == 0) {
        predicted_value += differences.last().unwrap();
        differences = get_differences(&differences);
    }
    return predicted_value;
}

fn get_differences(numbers: &[i64]) -> Vec<i64> {
    let length_of_differences = numbers.len() - 1;
    let mut differences = Vec::new();
    for i in 0..length_of_differences {
        let difference = numbers[i + 1] - numbers[i];
        differences.push(difference);
    }
    return differences;
}

fn solve_p2(input: &str) -> i64 {
    let mut result: i64 = 0;
    for line in input.lines() {
        let numbers = string_to_int_vect(line);
        result += predict_prev_value(&numbers);
    }
    return result;
}

fn predict_prev_value(numbers: &[i64]) -> i64 {
    let differences = get_differences(numbers);

    if differences.iter().all(|&x| x == 0) {
        return *numbers.first().unwrap();
    } else {
        let first_value = numbers.first().unwrap();
        let difference = predict_prev_value(&differences);
        return first_value - difference;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_sample() {
        let example_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = solve_p1(example_input);
        let expected = 114;
        assert_eq!(result, expected);
    }

    #[test]
    fn p2_sample() {
        let example_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = solve_p2(example_input);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_predict_next_value() {
        let line1 = [0, 3, 6, 9, 12, 15];
        let line2 = [1, 3, 6, 10, 15, 21];
        let line3 = [10, 13, 16, 21, 30, 45];

        assert_eq!(predict_next_value(&line1), 18);
        assert_eq!(predict_next_value(&line2), 28);
        assert_eq!(predict_next_value(&line3), 68);
    }

    #[test]
    fn test_predict_prev_value() {
        let line1 = [0, 3, 6, 9, 12, 15];
        let line2 = [1, 3, 6, 10, 15, 21];
        let line3 = [10, 13, 16, 21, 30, 45];

        assert_eq!(predict_prev_value(&line1), -3);
        assert_eq!(predict_prev_value(&line2), 0);
        assert_eq!(predict_prev_value(&line3), 5);
    }

    #[test]
    fn test_get_difference() {
        let line1 = [0, 3, 6, 9, 12, 15];
        let line2 = [3, 3, 3, 3, 3];
        let line3 = [0, 0, 0, 0];

        assert_eq!(get_differences(&line1), line2);
        assert_eq!(get_differences(&line2), line3);
    }
}
