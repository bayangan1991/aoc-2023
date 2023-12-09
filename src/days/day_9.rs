use std::collections::HashSet;

pub fn exec(source: &str) -> (isize, isize) {
    let part_a = source
        .split('\n')
        .map(|line| predict_next(&parse_line(line, false)))
        .sum();

    let part_b = source
        .split('\n')
        .map(|line| predict_next(&parse_line(line, true)))
        .sum();

    (part_a, part_b)
}

fn parse_line(line: &str, reverse: bool) -> Vec<isize> {
    let result = line.split_whitespace().map(|p| p.parse().unwrap());

    if reverse {
        result.rev().collect()
    } else {
        result.collect()
    }
}

fn predict_next(items: &Vec<isize>) -> isize {
    let last = *items.last().unwrap();
    let diffs = items
        .windows(2)
        .map(|i| i[1] - i[0])
        .collect::<Vec<isize>>();

    let uniques: HashSet<isize> = HashSet::from_iter(diffs.iter().map(|&a| a));

    let diff = *diffs.last().unwrap();

    if uniques.len() == 1 {
        last + diff
    } else {
        let next = predict_next(&diffs);
        last + next
    }
}

#[allow(dead_code)]
fn pairwise<I>(items: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I: IntoIterator + Clone,
{
    let right = items.clone().into_iter().skip(1);
    items.into_iter().zip(right)
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1 2 3", false), vec![1, 2, 3]);
    }

    #[test]
    fn test_sample() {
        let sample = read_input("9_sample_1");
        assert_eq!(exec(&sample).0, 114);
    }

    #[test]
    fn test_pairwise() {
        let items = vec![1, 2, 3];
        let result = pairwise(items.iter()).collect::<Vec<(&i32, &i32)>>();
        assert_eq!(result, vec![(&1, &2), (&2, &3)])
    }

    #[test]
    fn test_predict_next_1() {
        let items = vec![1, 2, 3, 4];
        assert_eq!(predict_next(&items), 5);
    }

    #[test]
    fn test_predict_next_2() {
        let items = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict_next(&items), 18);
    }

    #[test]
    fn test_predict_next_3() {
        let items = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(predict_next(&items), 28);
    }
    #[test]
    fn test_predict_next_4() {
        let items = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict_next(&items), 68);
    }
}
