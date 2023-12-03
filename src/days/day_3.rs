#[derive(PartialEq, Debug, Clone)]
struct Point(i32, i32);

#[derive(PartialEq, Debug, Clone)]
struct Part {
    number: String,
    coord: Point,
}

impl Part {
    fn value(&self) -> i32 {
        self.number.parse().unwrap()
    }
    fn in_range(&self, point: &Point) -> bool {
        let min_x = self.coord.0 - 1;
        let max_x = self.coord.0 + self.number.chars().count() as i32;
        let min_y = self.coord.1 - 1;
        let max_y = self.coord.1 + 1;

        point.0 >= min_x && point.0 <= max_x && point.1 >= min_y && point.1 <= max_y
    }
}

pub fn exec(source: &String, part: i32) -> i32 {
    let (points, parts) = get_parts_and_points(source);

    match part {
        1 => parts
            .iter()
            .map(|part| {
                if points.iter().any(|point| part.in_range(point)) {
                    part.value()
                } else {
                    0
                }
            })
            .sum(),
        _ => panic!("How did we end up here?"),
    }
}

fn parse_line(line: &str, line_index: i32) -> (Vec<Point>, Vec<Part>) {
    let mut parts = vec![];
    let mut points = vec![];
    let mut current_part = Part {
        number: String::new(),
        coord: Point(-1, line_index),
    };

    for (index, char) in line.chars().enumerate() {
        if char.is_numeric() {
            current_part.number.push(char);
            if current_part.coord.0 == -1 {
                current_part.coord = Point(index as i32, line_index);
            }
        } else if !char.is_ascii_digit() {
            if char != '.' {
                points.push(Point(index as i32, line_index));
            }
            if current_part.coord.0 != -1 {
                parts.push(current_part.clone());
                current_part = Part {
                    number: String::new(),
                    coord: Point(-1, line_index),
                };
            }
        }
    }

    if current_part.coord.0 != -1 {
        parts.push(current_part.clone());
    }

    (points, parts)
}

fn get_parts_and_points(source: &str) -> (Vec<Point>, Vec<Part>) {
    let mut points = vec![];
    let mut parts = vec![];

    for (line_index, line) in source.split('\n').enumerate() {
        let (new_points, new_parts) = parse_line(line, line_index as i32);
        points.extend(new_points);
        parts.extend(new_parts);
    }

    (points, parts)
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("123..*", 0),
            (
                vec![Point(5, 0)],
                vec![Part {
                    number: String::from("123"),
                    coord: Point(0, 0),
                }]
            )
        )
    }

    #[test]
    fn test_parse_line_2() {
        assert_eq!(
            parse_line("123..456", 0),
            (
                vec![],
                vec![
                    Part {
                        number: String::from("123"),
                        coord: Point(0, 0),
                    },
                    Part {
                        number: String::from("456"),
                        coord: Point(5, 0),
                    },
                ]
            )
        );
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            get_parts_and_points("123...*\n.456.#."),
            (
                vec![Point(6, 0), Point(5, 1)],
                vec![
                    Part {
                        number: String::from("123"),
                        coord: Point(0, 0),
                    },
                    Part {
                        number: String::from("456"),
                        coord: Point(1, 1),
                    },
                ]
            )
        )
    }

    #[test]
    fn test_in_range() {
        let part = Part {
            number: String::from("123"),
            coord: Point(0, 0),
        };
        // In Range
        assert!(part.in_range(&Point(0, 1))); // 1 below
        assert!(part.in_range(&Point(-1, 0))); // 1 left
        assert!(part.in_range(&Point(3, 0))); // 1 right
        assert!(part.in_range(&Point(0, -1))); // 1 above

        assert!(part.in_range(&Point(-1, -1))); // diagonal
        assert!(part.in_range(&Point(-1, 1))); // diagonal
        assert!(part.in_range(&Point(3, -1))); // diagonal
        assert!(part.in_range(&Point(3, 1))); // diagonal

        // Out of Range
        assert!(!part.in_range(&Point(0, 2))); // 2 below
        assert!(!part.in_range(&Point(-2, 0))); // 2 left
        assert!(!part.in_range(&Point(4, 0))); // 2 right
        assert!(!part.in_range(&Point(0, -2))); // 2 above

        assert!(!part.in_range(&Point(-2, -1))); // diagonal
        assert!(!part.in_range(&Point(-2, 1))); // diagonal
        assert!(!part.in_range(&Point(4, -1))); // diagonal
        assert!(!part.in_range(&Point(4, 1))); // diagonal

        assert!(!part.in_range(&Point(-1, -2))); // diagonal
        assert!(!part.in_range(&Point(-1, 2))); // diagonal
        assert!(!part.in_range(&Point(3, -2))); // diagonal
        assert!(!part.in_range(&Point(3, 2))); // diagonal
    }

    #[test]
    fn test_part_value() {
        let part = Part {
            number: String::from("123"),
            coord: Point(0, 0),
        };

        assert_eq!(part.value(), 123);
    }

    #[test]
    fn test_example_1() {
        assert_eq!(exec(&String::from("123.*"), 1), 0);
        assert_eq!(exec(&String::from("123*."), 1), 123);
        assert_eq!(exec(&String::from("123..\n*...."), 1), 123);
        assert_eq!(exec(&String::from("123..\n.....\n*...."), 1), 0);
        assert_eq!(exec(&String::from("123..\n...*.\n....."), 1), 123);
        assert_eq!(exec(&String::from("123..\n...*.\n..456"), 1), 579);
        assert_eq!(exec(&String::from("123..\n.*.*.\n..456"), 1), 579);
        assert_eq!(exec(&String::from("123..\n....*\n..456"), 1), 456);
        assert_eq!(exec(&String::from("111\n*$*\n1.1"), 1), 113);
        assert_eq!(exec(&String::from("111\n*..\n1.1"), 1), 112);
        assert_eq!(exec(&String::from("111\n...\n1*1"), 1), 2);
    }

    #[test]
    fn test_sample_data() {
        let sample_data = read_input("3_sample_1");
        assert_eq!(exec(&sample_data, 1), 4361);
    }

    #[test]
    fn test_validate_first_three_lines() {
        let data = read_input("3");
        let sample_data = data.split("\n").collect::<Vec<&str>>();

        let first_3_lines = &sample_data[..3].join("\n");

        assert_eq!(exec(&first_3_lines, 1), 8264);
    }

    #[test]
    fn test_excluded_parts() {
        let sample_data = read_input("3_sample_1");
        let (points, parts) = get_parts_and_points(&sample_data);

        let mut excluded_parts: Vec<Part> = vec![];

        parts.iter().for_each(|part| {
            if points.iter().any(|point| part.in_range(&point)) {
            } else {
                excluded_parts.push(part.clone())
            };
        });

        assert_eq!(
            excluded_parts,
            vec![
                Part {
                    number: String::from("114"),
                    coord: Point(5, 0),
                },
                Part {
                    number: String::from("58"),
                    coord: Point(7, 5),
                },
            ]
        )
    }
}
