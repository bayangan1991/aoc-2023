use std::collections::HashMap;

fn parse_digit(item: &str, digit_map: &HashMap<&str, &str>) -> char {
    if digit_map.contains_key(item) {
        *digit_map.get(item).unwrap()
    } else {
        item
    }
    .chars()
    .next()
    .unwrap()
}

fn parse_line(line: &str, to_find: &Vec<&str>, digit_map: &HashMap<&str, &str>) -> i32 {
    let mut min_index = 999;
    let mut max_index = 0;
    let mut min_digit = '0';
    let mut max_digit = '0';

    for test in to_find {
        match line.find(test) {
            None => {}
            Some(index) => {
                if index < min_index {
                    min_index = index;
                    min_digit = parse_digit(test, digit_map);
                }
            }
        };
        match line.rfind(test) {
            None => {}
            Some(index) => {
                if index >= max_index {
                    max_index = index;
                    max_digit = parse_digit(test, digit_map);
                }
            }
        }
    }
    format!("{}{}", min_digit, max_digit).parse().unwrap()
}

pub fn exec(source: &str) -> (i32, i32) {
    let lines = source.split('\n').collect::<Vec<_>>();

    let digit_map = HashMap::from([
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

    let values: Vec<_> = digit_map.values().cloned().collect();
    let values_2 = [values.clone(), digit_map.keys().cloned().collect()].concat();

    let (part_1, part_2) = lines
        .iter()
        .map(|line| {
            (
                parse_line(line, &values, &digit_map),
                parse_line(line, &values_2, &digit_map),
            )
        })
        .unzip::<i32, i32, Vec<_>, Vec<_>>();

    (part_1.iter().sum(), part_2.iter().sum())
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_sample_data_1() {
        let sample_data = read_input("1_sample_1");
        assert_eq!(exec(&sample_data), (142, 142));
    }

    #[test]
    fn test_sample_data_2() {
        let sample_data = read_input("1_sample_2");
        assert_eq!(exec(&sample_data), (209, 281));
    }

    #[test]
    fn test_line_parse_1() {
        let sample_data = String::from("eightone7threenl7mtxbmkpkzqzljrdk");
        assert_eq!(exec(&sample_data), (77, 87))
    }

    #[test]
    fn test_line_parse_2() {
        let sample_data = String::from("hzgrkrbmjmzhpfkfgg5");
        assert_eq!(exec(&sample_data), (55, 55))
    }

    #[test]
    fn test_line_parse_3() {
        let sample_data = String::from("1");
        assert_eq!(exec(&sample_data), (11, 11))
    }
}
