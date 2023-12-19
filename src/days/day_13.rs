use std::collections::HashMap;

use itertools::min;

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
enum Type {
    Ash,
    Rock,
}

pub fn exec(source: &str) -> (usize, usize) {
    let grids = parse_grids(source);

    let mut part_a = 0;
    let mut part_b = 0;

    grids.iter().for_each(|grid| {
        let (l, r) = find_rotating_symmetry_line(grid);
        part_a += l.unwrap();
        part_b += r.unwrap();
    });

    (part_a, part_b)
}

fn parse_grids(source: &str) -> Vec<Vec<Vec<Type>>> {
    source.split("\n\n").map(|grid| parse_grid(grid)).collect()
}

fn parse_grid(grid: &str) -> Vec<Vec<Type>> {
    grid.split('\n').map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Vec<Type> {
    line.chars()
        .map(|c| match c {
            '#' => Type::Rock,
            '.' => Type::Ash,
            _ => panic!("Oopsie Whoopsie"),
        })
        .collect()
}

fn find_rotating_symmetry_line(grid: &Vec<Vec<Type>>) -> (Option<usize>, Option<usize>) {
    match (
        find_symmetry_line(&grid),
        find_symmetry_line(&transpose(&grid)),
    ) {
        ((Some(a), Some(b)), (None, None)) => (Some(a), Some(b)),
        ((Some(a), None), (None, Some(b))) => (Some(a), Some(b * 100)),
        ((None, Some(b)), (Some(a), None)) => (Some(a * 100), Some(b)),
        ((None, None), (Some(a), Some(b))) => (Some(a * 100), Some(b * 100)),
        _ => panic!("Shouldn't be possible"),
    }
}

fn find_symmetry_line(grid: &Vec<Vec<Type>>) -> (Option<usize>, Option<usize>) {
    let mut result: HashMap<usize, usize> = HashMap::new();

    for line in grid {
        let set = find_symmetry_lines(line);

        for (i, diffs) in set {
            result.entry(i).and_modify(|e| *e += diffs).or_insert(diffs);
        }
    }

    let mut zero = None;
    let mut one = None;

    for (i, diff) in result {
        if diff == 0 {
            zero = Some(i)
        } else if diff == 1 {
            one = Some(i)
        }
    }

    (zero, one)
}

fn find_symmetry_lines(line: &Vec<Type>) -> HashMap<usize, usize> {
    let mut result = HashMap::new();

    for i in 0..line.len() {
        let (left, right) = line.split_at(i);
        let cmp_size = min([left.len(), right.len()]).unwrap();
        if cmp_size == 0 {
            continue;
        }
        let (_, left) = left.split_at(i - cmp_size);
        let (right, _) = right.split_at(cmp_size);

        let diffs: usize = left
            .iter()
            .zip(right.iter().rev())
            .map(|(l, r)| if l == r { 0 } else { 1 })
            .sum();

        result.insert(i, diffs);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_sample_data_1() {
        let sample_data = read_input("13_sample_1");
        assert_eq!(exec(&sample_data), (405, 400));
    }

    #[test]
    fn test_symmetry_finder() {
        let line = parse_line("#....#");

        let result = find_symmetry_lines(&line);

        assert!(*result.get(&3).unwrap() == 0);
    }

    #[test]
    fn test_find_symmetry_line() {
        let sample = parse_grid(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );

        assert_eq!(find_symmetry_line(&sample), (Some(5), None))
    }

    #[test]
    fn test_find_transposed_symmetry_line() {
        let sample = parse_grid(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );

        assert_eq!(find_rotating_symmetry_line(&sample), (Some(400), Some(100)))
    }
}

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = vec![Vec::new(); matrix.iter().next().unwrap().len()];

    for row in matrix.iter() {
        for (index, item) in row.iter().enumerate() {
            transposed[index].push(item.clone());
        }
    }

    transposed
}
