use cached::proc_macro::cached;
use itertools::Itertools;

pub fn exec(source: &str) -> (usize, usize) {
    let part_a = source
        .split("\n")
        .map(|line| {
            let line = parse_line(line);
            solve(String::from(line.0), line.1, false)
        })
        .sum();

    let part_b = source
        .split("\n")
        .map(|line| {
            let line = parse_line(line);
            let left = vec![line.0].repeat(5).join("?");
            solve(left, line.1.repeat(5), false)
        })
        .sum();

    (part_a, part_b)
}

#[cached]
fn solve(line: String, arrangement: Vec<usize>, in_match: bool) -> usize {
    let matches: usize = arrangement.iter().sum();
    if line.is_empty() {
        return if matches == 0 { 1 } else { 0 };
    }

    let remaining = *arrangement.get(0).unwrap_or(&0);
    let (_, new_line) = line.split_at(1);
    let substr = String::from(new_line);
    let mut new_arrangement = arrangement.iter().skip(1).map(|i| *i).collect_vec();

    match line.chars().next() {
        None => {}
        Some('#') => {
            if remaining == 0 {
                return 0;
            }

            new_arrangement.insert(0, remaining - 1);
            return solve(substr, new_arrangement, true);
        }
        Some('.') => {
            if in_match && remaining != 0 {
                return 0;
            } else if remaining == 0 {
                return solve(substr, new_arrangement, false);
            } else {
                return solve(substr, arrangement, false);
            }
        }
        Some('?') => {
            let a = format!("#{}", substr);
            let b = format!(".{}", substr);
            return solve(a, arrangement.clone(), in_match) + solve(b, arrangement, in_match);
        }
        _ => panic!("boo"),
    }

    0
}

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let (left, right) = line.split_once(' ').unwrap();
    (left, right.split(',').map(|c| c.parse().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_sample_data_1() {
        let sample_data = read_input("12_sample_1");
        assert_eq!(exec(&sample_data).0, 21);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("#.#.### 1,1,3"), ("#.#.###", vec![1, 1, 3]));
    }
}
