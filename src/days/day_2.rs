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
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for m in &self.matches {
            red = if m.red > red { m.red } else { red };
            green = if m.green > green { m.green } else { green };
            blue = if m.blue > blue { m.blue } else { blue };
        }

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
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for pull in result.split(", ") {
            let (num_str, colour) = pull.split_once(" ").unwrap();
            let num = num_str.parse::<i32>().unwrap();
            match colour {
                "red" => red += num,
                "green" => green += num,
                "blue" => blue += num,
                _ => panic!("Whoops"),
            }
        }
        game.matches.push(Match { red, green, blue })
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
    let games = source.split("\n").map(|line| parse_line(line));
    match part {
        1 => games
            .map(|game| {
                if game_is_valid(&game, 12, 13, 14) {
                    game.id
                } else {
                    0
                }
            })
            .sum(),
        2 => games.map(|game| game.max_stones().power()).sum(),
        _ => panic!("Uhh uh uh"),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;
    use std::panic::catch_unwind;

    use super::*;

    #[test]
    fn test_bad_part() {
        let result = catch_unwind(|| exec(&String::from("test"), 3));
        assert!(result.is_err());
    }

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
