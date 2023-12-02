use std::collections::HashMap;

fn parse_digit(item: &str, digit_map: &HashMap<&str, &str>) -> String {
    if digit_map.contains_key(item) {
        String::from(*digit_map.get(item).unwrap())
    } else {
        String::from(item)
    }
}

fn parse_line(line: &str, to_find: &Vec<&str>, digit_map: &HashMap<&str, &str>) -> i32 {
    let mut min_index = 999;
    let mut max_index = 0;
    let mut min_digit = String::new();
    let mut max_digit = String::new();

    for test in to_find {
        match line.find(test) {
            None => {}
            Some(index) => {
                if index < min_index {
                    min_index = index;
                    min_digit = parse_digit(&test, &digit_map);
                }
            }
        };
        match line.rfind(test) {
            None => {}
            Some(index) => {
                if index >= max_index {
                    max_index = index;
                    max_digit = parse_digit(&test, &digit_map);
                }
            }
        }
    }
    format!("{}{}", min_digit, max_digit).parse().unwrap()
}

pub fn exec(source: &str, part: i32) -> i32 {
    let lines = source.split('\n');

    let digit_map: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let values = digit_map.values().cloned().collect::<Vec<&str>>();

    let to_find = match part {
        1 => values,
        2 => [values, digit_map.keys().cloned().collect::<Vec<&str>>()].concat(),
        _ => panic!("Polly shouldn't be"),
    };

    lines
        .map(|line| parse_line(line, &to_find, &digit_map))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_bad_part() {
        let result = catch_unwind(|| exec(&String::from("test"), 3));
        assert!(result.is_err());
    }

    #[test]
    fn test_sample_data_1() {
        let sample_data = read_input("1_sample_1");
        assert_eq!(exec(&sample_data, 1), 142);
    }

    #[test]
    fn test_sample_data_2() {
        let sample_data = read_input("1_sample_2");
        assert_eq!(exec(&sample_data, 2), 281);
    }

    #[test]
    fn test_line_parse_1() {
        let sample_data = String::from("eightone7threenl7mtxbmkpkzqzljrdk");
        assert_eq!(exec(&sample_data, 2), 87)
    }

    #[test]
    fn test_line_parse_2() {
        let sample_data = String::from("hzgrkrbmjmzhpfkfgg5");
        assert_eq!(exec(&sample_data, 2), 55)
    }

    #[test]
    fn test_line_parse_3() {
        let sample_data = String::from("1");
        assert_eq!(exec(&sample_data, 2), 11)
    }
}
