use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
enum Space {
    Galaxy,
    Nothing,
}

pub fn exec(source: &str) -> (usize, usize) {
    let data = source
        .split('\n')
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();

    (calculate(&data, 1), calculate(&data, 999999))
}

fn calculate(map: &Vec<Vec<Space>>, dilate_by: usize) -> usize {
    let data = map.clone();
    let mut galaxies = HashMap::new();

    for (y, line) in data.iter().enumerate() {
        for (x, item) in line.iter().enumerate() {
            match item {
                Space::Galaxy => {
                    galaxies.insert((x, y), (x, y));
                }
                _ => {}
            }
        }
    }

    insert_blanks(&data, &mut galaxies, dilate_by, false);
    let data = transpose(&data);
    insert_blanks(&data, &mut galaxies, dilate_by, true);

    let mut result = 0;

    let galaxies = galaxies.values().collect::<Vec<_>>();

    for (index, &&left) in galaxies.iter().enumerate() {
        for &&right in galaxies.iter().skip(index + 1) {
            result += distance(left, right);
        }
    }

    result
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn insert_blanks(
    space: &Vec<Vec<Space>>,
    galaxies: &mut HashMap<(usize, usize), (usize, usize)>,
    dilate_by: usize,
    tranposed: bool,
) {
    let mut blanks = vec![];

    for (index, line) in space.iter().enumerate().rev() {
        if line.iter().all(|p| p == &Space::Nothing) {
            blanks.push(index);
        }
    }

    let mut dilate_amount = 0;
    for (y, line) in space.iter().enumerate() {
        if blanks.contains(&y) {
            dilate_amount += 1;
        }
        for (x, _) in line.iter().enumerate() {
            let key = if tranposed { (y, x) } else { (x, y) };

            match galaxies.get_mut(&key) {
                None => {}
                Some(entry) => {
                    if tranposed {
                        entry.0 += dilate_by * dilate_amount;
                    } else {
                        entry.1 += dilate_by * dilate_amount;
                    }
                }
            }
        }
    }
}

fn transpose(matrix: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let mut transposed = vec![Vec::new(); matrix.len()];

    for row in matrix.iter() {
        for (index, item) in row.iter().enumerate() {
            transposed[index].push(item.clone());
        }
    }

    transposed
}

fn parse_line(line: &str) -> Vec<Space> {
    line.chars()
        .map(|c| match c {
            '.' => Space::Nothing,
            '#' => Space::Galaxy,
            _ => panic!("Not space"),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line(".#."),
            vec![Space::Nothing, Space::Galaxy, Space::Nothing]
        );
    }

    #[test]
    fn test_insert_blanks() {
        let sample = vec![
            vec![Space::Nothing, Space::Nothing],
            vec![Space::Nothing, Space::Nothing],
            vec![Space::Nothing, Space::Galaxy],
        ];
        let mut galaxies = HashMap::from([((1, 2), (1, 2))]);
        insert_blanks(&sample, &mut galaxies, 1, false);
        let x = galaxies.get(&(1, 2)).unwrap();
        assert_eq!(x, &(1, 4));
    }

    #[test]
    fn test_rotate() {
        let sample = vec![
            vec![Space::Nothing, Space::Nothing],
            vec![Space::Galaxy, Space::Galaxy],
        ];
        let expected = vec![
            vec![Space::Nothing, Space::Galaxy],
            vec![Space::Nothing, Space::Galaxy],
        ];

        assert_eq!(transpose(&sample), expected);
    }

    #[test]
    fn test_sample() {
        let sample = read_input("11_sample_1");
        assert_eq!(exec(&sample), (374, 82000210));
    }
}
