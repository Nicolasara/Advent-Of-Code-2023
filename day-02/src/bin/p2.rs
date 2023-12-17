use std::{cmp::max, fs::read_to_string};

fn main() {
    let path = "src/input.txt";
    let input = read_to_string(path).unwrap();
    let output = solve(input.as_str());
    println!("Result: {}", output);
}

fn solve(input: &str) -> i64 {
    let mut response = 0;
    for line in input.lines() {
        let mut parts = line.split(':');
        let game_input = parts.nth(1).unwrap();
        response += power_of_cubes(game_input);
    }
    return response;
}

fn power_of_cubes(game_input: &str) -> i64 {
    let handfuls = game_input.split(';');

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for handful in handfuls {
        let (red, green, blue) = red_green_blue_count(handful);
        max_red = max(max_red, red);
        max_green = max(max_green, green);
        max_blue = max(max_blue, blue);
    }
    return max_red * max_green * max_blue;
}

fn red_green_blue_count(handful_of_cubes: &str) -> (i64, i64, i64) {
    let mut red_count = 0;
    let mut green_count = 0;
    let mut blue_count = 0;
    let amount_color_pairs = handful_of_cubes.split(',');
    for amount_color_pair in amount_color_pairs {
        let mut parts = amount_color_pair.split_whitespace();
        let amount: i64 = parts.nth(0).unwrap().parse().unwrap();
        let color = parts.nth(0).unwrap();
        match color {
            "red" => red_count += amount,
            "green" => green_count += amount,
            "blue" => blue_count += amount,
            &_ => panic!("unknown color {}", color),
        }
    }
    return (red_count, green_count, blue_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = solve(example_input);
        let expected = 2286;
        assert_eq!(result, expected);
    }
}
