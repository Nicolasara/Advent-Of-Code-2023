use std::fs::read_to_string;

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
        let game_id = parts.nth(0).unwrap();
        let game_input = parts.nth(0).unwrap();
        if game_is_possible(game_input) {
            response += get_game_id(game_id);
        }
    }
    return response;
}

fn game_is_possible(game_input: &str) -> bool {
    let handfuls = game_input.split(';');

    for handful in handfuls {
        if number_of_tiles_inconsistent(handful, 12, 13, 14) {
            return false;
        }
    }
    return true;
}

fn number_of_tiles_inconsistent(
    handful_of_cubes: &str,
    red_in_bag: i64,
    green_in_bag: i64,
    blue_in_bag: i64,
) -> bool {
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
    let red_inconsistent = red_count > red_in_bag;
    let green_inconsistent = green_count > green_in_bag;
    let blue_inconsistent = blue_count > blue_in_bag;
    return red_inconsistent || green_inconsistent || blue_inconsistent;
}

fn get_game_id(line: &str) -> i64 {
    let game_id: Option<&str> = line.split_whitespace().nth(1);
    return game_id.unwrap().parse().unwrap();
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
        let expected = 8;
        assert_eq!(result, expected);
    }
}
