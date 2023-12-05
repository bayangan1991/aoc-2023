#[derive(PartialEq, Debug)]
struct Game {
    id: i32,
    matches: Vec<Match>,
}

#[derive(PartialEq, Debug)]
struct Match {
    red: i32,
    green: i32,
    blue: i32,
}

impl Match {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

impl Game {
    fn max_stones(&self) -> Match {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for m in &self.matches {
            red = if m.red > red { m.red } else { red };
            green = if m.green > green { m.green } else { green };
            blue = if m.blue > blue { m.blue } else { blue };
        }

        Match { red, green, blue }
    }

    fn is_valid(&self) -> bool {
        let stones = self.max_stones();
        return stones.red <= 12 && stones.green <= 13 && stones.blue <= 14;
    }
}

fn parse_line(line: &str) -> Game {
    let (left, right) = line.split_once(": ").unwrap();
    let id = left[5..].parse().unwrap();
    let mut matches = vec![];

    for result in right.split("; ") {
        let (mut red, mut green, mut blue) = (0, 0, 0);

        for pull in result.split(", ") {
            let (num, colour) = pull.split_once(" ").unwrap();
            let num = num.parse::<i32>().unwrap();
            match colour {
                "red" => red += num,
                "green" => green += num,
                "blue" => blue += num,
                _ => panic!("Whoops"),
            }
        }
        matches.push(Match { red, green, blue })
    }

    Game { id, matches }
}

pub fn exec(source: &String) -> (i32, i32) {
    let games = source
        .split("\n")
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let (part_1, part_2) = games
        .iter()
        .map(|game| {
            (
                if game.is_valid() { game.id } else { 0 },
                game.max_stones().power(),
            )
        })
        .unzip::<i32, i32, Vec<_>, Vec<_>>();

    (part_1.iter().sum(), part_2.iter().sum())
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;
    use std::panic::catch_unwind;

    use super::*;

    #[test]
    fn test_part_1_with_sample() {
        let sample_data = read_input("2_sample_1");
        assert_eq!(exec(&sample_data), (8, 2286))
    }

    #[test]
    fn test_bad_colour() {
        let result = catch_unwind(|| parse_line("Game 1: 3 purple"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                matches: vec![
                    Match {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Match {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Match {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ]
            }
        );
    }
}
