use scan_fmt::scan_fmt;
use std::{collections::HashMap, fs::read_to_string};

struct Card {
    index: i64,
    winning_numbers: Vec<i64>,
    players_numbers: Vec<i64>,
}

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
        let card = parse_card(line);
        result += get_cards_point_value(card);
    }

    return result;
}

fn parse_card(input: &str) -> Card {
    let card_parts = parse_card_parts(input);
    let index = parse_card_index(card_parts[0]);
    let winning_numbers = parse_winning_numbers(card_parts[1].trim());
    let players_numbers = parse_players_numbers(card_parts[2].trim());
    return Card {
        index,
        winning_numbers,
        players_numbers,
    };
}

fn parse_card_index(input: &str) -> i64 {
    let card_index = scan_fmt!(input, "Card {}", i64).unwrap();
    return card_index;
}

fn parse_winning_numbers(input: &str) -> Vec<i64> {
    let winning_numbers: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    return winning_numbers.try_into().unwrap();
}

fn parse_players_numbers(input: &str) -> Vec<i64> {
    let players_numbers: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    return players_numbers;
}

fn parse_card_parts(input: &str) -> [&str; 3] {
    let parts: Vec<&str> = input.split(": ").collect();
    if parts.len() != 2 {
        panic!("card is incorrectly formatted when splitting \": \"")
    }

    let card_numbers: Vec<&str> = parts[1].split("| ").collect();
    if card_numbers.len() != 2 {
        panic!("card is incorrectly formatted when splitting \"| \"")
    }

    return [parts[0], card_numbers[0], card_numbers[1]];
}

fn get_cards_point_value(card: Card) -> i64 {
    let mut points = 0;
    for players_number in card.players_numbers {
        if contains_number(&card.winning_numbers, &players_number) {
            if points == 0 {
                points = 1;
            } else {
                points = points * 2;
            }
        }
    }
    return points;
}

fn contains_number(list: &[i64], number: &i64) -> bool {
    for number_in_list in list {
        if number_in_list == number {
            return true;
        }
    }
    return false;
}

fn solve_p2(input: &str) -> i64 {
    let cards: Vec<Card> = input.lines().map(|line| parse_card(line)).collect();
    let mut card_counts = HashMap::new();
    for card in &cards {
        card_counts.insert(card.index, 1);
    }

    for card in &cards {
        let current_card_count = *card_counts.get(&card.index).unwrap();
        let cards_to_copy = get_cards_to_copy(&card);
        for card_to_copy in cards_to_copy {
            card_counts.insert(
                card_to_copy,
                card_counts.get(&card_to_copy).unwrap_or(&0) + current_card_count,
            );
        }
    }
    return card_counts.values().sum();
}

fn get_cards_to_copy(card: &Card) -> Vec<i64> {
    let mut count = 0;
    for players_number in &card.players_numbers {
        if contains_number(&card.winning_numbers, players_number) {
            count += 1;
        }
    }
    let mut card_copy_indexes = Vec::new();
    let next_card_index = card.index + 1;
    for card_index in next_card_index..next_card_index + count {
        card_copy_indexes.push(card_index);
    }
    return card_copy_indexes;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sample<T, F>(solve_fn: T, input: &str, expected: F)
    where
        F: PartialEq + std::fmt::Debug,
        T: Fn(&str) -> F,
    {
        let result = solve_fn(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn p1_sample() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        test_sample(solve_p1, input, 13);
    }

    #[test]
    fn p2_sample() {
        let input1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let input2 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let input3 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let input4 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let input5 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let input6 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        test_sample(solve_p2, input1, 5);
        test_sample(solve_p2, input2, 10);
        test_sample(solve_p2, input3, 19);
        test_sample(solve_p2, input4, 28);
        test_sample(solve_p2, input5, 29);
        test_sample(solve_p2, input6, 30);
    }
}
