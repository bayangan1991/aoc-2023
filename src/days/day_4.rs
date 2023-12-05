use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Clone, Debug)]
struct Card {
    id: u32,
    matches: u32,
}

pub fn exec(source: &str, part: u32) -> u32 {
    let cards: Vec<_> = source
        .split('\n')
        .enumerate()
        .map(|(index, line)| parse_line(index as u32 + 1, line))
        .collect();
    match part {
        1 => cards.iter().map(|card| calc_score(&card)).sum(),
        2 => calc_part_2(&cards),
        _ => panic!("Polly shouldn't be"),
    }
}

fn parse_line(id: u32, line: &str) -> Card {
    let (_, game) = line.split_once(": ").unwrap();
    let (ticket_raw, win_raw) = game.split_once(" | ").unwrap();
    let matches = extract_numbers(ticket_raw)
        .intersection(&extract_numbers(win_raw))
        .count() as u32;

    Card { id, matches }
}

fn extract_numbers(number_string: &str) -> HashSet<u32> {
    let mut result = HashSet::new();
    let mut current_number = String::new();

    for char in number_string.trim().chars() {
        if char.is_ascii_digit() {
            current_number.push(char);
        } else {
            if current_number.as_str() != "" {
                result.insert(current_number.parse().unwrap());
                current_number = String::new();
            }
        }
    }

    if current_number.as_str() != "" {
        result.insert(current_number.parse().unwrap());
    }

    result
}

fn calc_score(card: &Card) -> u32 {
    if card.matches > 0 {
        2_u32.pow(card.matches - 1)
    } else {
        0
    }
}

fn calc_part_2(cards: &Vec<Card>) -> u32 {
    let mut card_counts = cards
        .iter()
        .map(|card| (card.id, 1_u32))
        .collect::<HashMap<_, _>>();

    for card in cards {
        let current = *card_counts.get(&card.id).unwrap();

        if card.matches > 0 {
            for x in &card.id + 1..=&card.id + card.matches {
                *card_counts.entry(x).or_insert(1) += current;
            }
        }
    }

    card_counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input;
    use std::panic::catch_unwind;

    #[test]
    fn test_bad_part() {
        let result = catch_unwind(|| exec(&String::from("test"), 3));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line() {
        let sample = "Game 1: 1 2 3 | 1 3";
        let result = parse_line(1, sample);
        assert_eq!(result.matches, 2);
    }

    #[test]
    fn test_parse_line_2() {
        let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = parse_line(1, sample);
        assert_eq!(result.matches, 4);
        assert_eq!(exec(sample, 1), 8);
    }

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers("1 2 3"), HashSet::from([1, 2, 3]));
        assert_eq!(extract_numbers(" 1 2 3"), HashSet::from([1, 2, 3]));
        assert_eq!(extract_numbers("1 2 3 "), HashSet::from([1, 2, 3]));
        assert_eq!(extract_numbers("1 2  3"), HashSet::from([1, 2, 3]));
        assert_eq!(extract_numbers(" 1    2  3"), HashSet::from([1, 2, 3]));
        assert_eq!(extract_numbers("1    2  3  "), HashSet::from([1, 2, 3]));
    }

    #[test]
    fn test_sample_data_1() {
        let sample_data = read_input("4_sample_1");
        assert_eq!(exec(&sample_data, 1), 13);
    }

    #[test]
    fn test_sample_data_2() {
        let sample_data = read_input("4_sample_1");
        assert_eq!(exec(&sample_data, 2), 30);
    }

    #[test]
    fn test_2_example() {
        let sample = vec![
            Card { id: 1, matches: 2 },
            Card { id: 2, matches: 1 },
            Card { id: 3, matches: 0 },
        ];

        assert_eq!(calc_part_2(&sample), 7)
    }
}
