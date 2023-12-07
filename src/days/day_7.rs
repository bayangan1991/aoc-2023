use std::cmp::Ordering;
use std::collections::HashMap;

const FACE_VALUE: [(char, usize); 5] = [('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10)];
const FACE_VALUE_2: [(char, usize); 5] = [('A', 14), ('K', 13), ('Q', 12), ('J', 1), ('T', 10)];

pub fn exec(source: &str) -> (usize, usize) {
    let hand_lookup = HashMap::from(FACE_VALUE);
    let hand_lookup_2 = HashMap::from(FACE_VALUE_2);

    let mut hands = source
        .split('\n')
        .map(|line| parse_line(line, &hand_lookup))
        .collect::<Vec<_>>();

    hands.sort_by(part_a);

    let part_1 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bet * (rank + 1))
        .collect::<Vec<_>>();

    let mut hands_2 = source
        .split('\n')
        .map(|line| parse_line(line, &hand_lookup_2))
        .collect::<Vec<_>>();

    hands_2.sort_by(part_b);

    let part_2 = hands_2
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bet * (rank + 1))
        .collect::<Vec<_>>();

    (part_1.iter().sum(), part_2.iter().sum())
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [usize; 5],
    bet: usize,
}

impl Hand {
    fn hand_rank(&self) -> usize {
        let mut counts = (2..=14)
            .map(|i| (self.cards.iter().filter(|j| **j == i).count(), i))
            .collect::<Vec<(usize, usize)>>();

        counts.sort();
        counts.reverse();

        let (count_1, _) = *counts.get(0).unwrap();
        let (count_2, _) = *counts.get(1).unwrap_or(&(0, 0));

        match (count_1, count_2) {
            (5, _) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, _) => 3,
            (2, 2) => 2,
            (2, _) => 1,
            _ => 0,
        }
    }
    fn hand_rank_b(&self) -> usize {
        let mut counts = (1..=14)
            .map(|i| (self.cards.iter().filter(|j| **j == i).count(), i))
            .collect::<Vec<(usize, usize)>>();

        let joker_count = self.cards.iter().filter(|j| **j == 1).count();

        counts.sort();
        counts.reverse();

        let (mut count_1, best_card) = *counts.get(0).unwrap();
        let (mut count_2, next_card) = *counts.get(1).unwrap_or(&(0, 0));

        if best_card != 1 {
            count_1 += joker_count;
        } else if next_card != 1 {
            count_2 += joker_count;
        }

        match (count_1, count_2) {
            (5, _) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, _) => 3,
            (2, 2) => 2,
            (2, _) => 1,
            _ => 0,
        }
    }
}

fn part_a(a: &Hand, b: &Hand) -> Ordering {
    match a.hand_rank().cmp(&b.hand_rank()) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            let mut ordering = Ordering::Equal;
            for (index, card) in a.cards.iter().enumerate() {
                ordering = card.cmp(&b.cards[index]);
                if ordering == Ordering::Equal {
                    continue;
                }
                break;
            }
            ordering
        }
    }
}

fn part_b(a: &Hand, b: &Hand) -> Ordering {
    match a.hand_rank_b().cmp(&b.hand_rank_b()) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            let mut ordering = Ordering::Equal;
            for (index, card) in a.cards.iter().enumerate() {
                ordering = card.cmp(&b.cards[index]);
                if ordering == Ordering::Equal {
                    continue;
                }
                break;
            }
            ordering
        }
    }
}

fn parse_line(line: &str, hand_lookup: &HashMap<char, usize>) -> Hand {
    let (left, right) = line.split_once(' ').unwrap();
    let bet = right.parse().unwrap();

    let cards = left
        .chars()
        .map(|char| match hand_lookup.get(&char) {
            None => char.to_digit(10).unwrap() as usize,
            Some(value) => *value,
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Hand { cards, bet }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_sample() {
        let sample = read_input("7_sample_1");
        assert_eq!(exec(&sample), (6440, 5905))
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("12345 678", &HashMap::from(FACE_VALUE)),
            Hand {
                cards: [1, 2, 3, 4, 5],
                bet: 678,
            }
        );
    }

    #[test]
    fn test_parse_line_with_faces() {
        assert_eq!(
            parse_line("T2A4K 678", &HashMap::from(FACE_VALUE)),
            Hand {
                cards: [10, 2, 14, 4, 13],
                bet: 678,
            }
        );
    }

    #[test]
    fn test_hand_ranks() {
        assert_eq!(
            Hand {
                cards: [2, 2, 2, 2, 2],
                bet: 1,
            }
            .hand_rank(),
            6
        );
        assert_eq!(
            Hand {
                cards: [2, 2, 3, 2, 2],
                bet: 1,
            }
            .hand_rank(),
            5
        );
        assert_eq!(
            Hand {
                cards: [2, 13, 2, 2, 13],
                bet: 1,
            }
            .hand_rank(),
            4
        );
        assert_eq!(
            Hand {
                cards: [2, 2, 2, 13, 10],
                bet: 1,
            }
            .hand_rank(),
            3
        );
        assert_eq!(
            Hand {
                cards: [2, 2, 7, 5, 5],
                bet: 1,
            }
            .hand_rank(),
            2
        );
        assert_eq!(
            Hand {
                cards: [2, 2, 7, 5, 9],
                bet: 1,
            }
            .hand_rank(),
            1
        );
        assert_eq!(
            Hand {
                cards: [2, 10, 7, 5, 9],
                bet: 1,
            }
            .hand_rank(),
            0
        );
    }

    #[test]
    fn test_sort_hands() {
        let mut hands = vec![
            Hand {
                cards: [13, 13, 13, 13, 13],
                bet: 1,
            },
            Hand {
                cards: [12, 12, 12, 12, 12],
                bet: 2,
            },
            Hand {
                cards: [3, 3, 3, 3, 12],
                bet: 3,
            },
            Hand {
                cards: [13, 13, 12, 12, 12],
                bet: 4,
            },
            Hand {
                cards: [11, 11, 11, 10, 10],
                bet: 5,
            },
            Hand {
                cards: [3, 3, 3, 2, 2],
                bet: 6,
            },
            Hand {
                cards: [12, 12, 8, 7, 5],
                bet: 7,
            },
        ];

        hands.sort_by(part_a);

        for hand in &hands {
            println!("{}", hand.hand_rank())
        }

        assert_eq!(hands[0].bet, 7);
        assert_eq!(hands[1].bet, 6);
        assert_eq!(hands[2].bet, 5);
        assert_eq!(hands[3].bet, 4);
        assert_eq!(hands[4].bet, 3);
        assert_eq!(hands[5].bet, 2);
        assert_eq!(hands[6].bet, 1);
    }
}
