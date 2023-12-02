use std::collections::HashMap;

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
        let red = self
            .matches
            .iter()
            .max_by(|m, other| m.red.cmp(&other.red))
            .unwrap()
            .red;
        let green = self
            .matches
            .iter()
            .max_by(|m, other| m.green.cmp(&other.green))
            .unwrap()
            .green;
        let blue = self
            .matches
            .iter()
            .max_by(|m, other| m.blue.cmp(&other.blue))
            .unwrap()
            .blue;

        Match { red, green, blue }
    }
}

fn parse_line(line: &str) -> Game {
    let (left, right) = line.split_once(": ").unwrap();

    let game_id = left[5..].parse::<i32>().unwrap();

    let mut game = Game {
        id: game_id,
        matches: vec![],
    };

    for result in right.split("; ") {
        let mut running_total: HashMap<&str, i32> =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for pull in result.split(", ") {
            let (num_str, colour) = pull.split_once(" ").unwrap();
            let number = num_str.parse::<i32>().unwrap();
            let mut target = running_total.get_mut(colour).unwrap();
            *target += number;
        }
        game.matches.push(Match {
            red: *running_total.get("red").unwrap(),
            green: *running_total.get("green").unwrap(),
            blue: *running_total.get("blue").unwrap(),
        })
    }

    game
}

fn game_is_valid(game: &Game, red: i32, green: i32, blue: i32) -> bool {
    game.matches.iter().all(|m| {
        m.red <= red
            && m.green <= green
            && m.blue <= blue
            && m.red + m.green + m.blue <= red + green + blue
    })
}

pub fn exec(source: &String, part: i32) -> i32 {
    match part {
        1 => source
            .split("\n")
            .map(|line| parse_line(line))
            .map(|game| {
                if game_is_valid(&game, 12, 13, 14) {
                    game.id
                } else {
                    0
                }
            })
            .sum(),
        2 => source
            .split("\n")
            .map(|line| parse_line(line))
            .map(|game| game.max_stones().power())
            .sum(),
        _ => panic!("Uhh uh uh"),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_part_1_with_sample() {
        let sample_data = read_input("2_sample_1");
        assert_eq!(exec(&sample_data, 1), 8)
    }
    #[test]
    fn test_part_2_with_sample() {
        let sample_data = read_input("2_sample_1");
        assert_eq!(exec(&sample_data, 2), 2286)
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
                        blue: 3
                    },
                    Match {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Match {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            }
        );
    }
}
