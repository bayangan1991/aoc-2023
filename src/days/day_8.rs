use std::collections::HashMap;

type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;

pub fn exec(source: &str) -> (usize, usize) {
    let (path, map) = parse_instructions(&source);
    let part_a = calc_steps("AAA", "ZZZ", &map, &path);

    let a_nodes = map
        .iter()
        .filter(|(&a, _)| a.ends_with("A"))
        .map(|(&a, _)| a)
        .collect::<Vec<_>>();

    let part_b = a_nodes
        .iter()
        .map(|&start| calc_steps(start, "Z", &map, &path))
        .collect::<Vec<_>>();

    let part_b = lcm_vec(&part_b);

    (part_a, part_b)
}

fn calc_steps(start: &str, end: &str, nodes: &NodeMap, path: &Vec<usize>) -> usize {
    let mut path = path.iter().cycle();
    let mut current_node = start;
    let mut steps = 0;

    loop {
        let next_step = path.next().unwrap();
        let (left, right) = nodes.get(&current_node).unwrap();
        current_node = match next_step {
            &0 => *left,
            &_ => *right,
        };

        steps += 1;

        if current_node.ends_with(end) {
            break;
        }
    }
    steps
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

fn lcm_vec(v: &Vec<usize>) -> usize {
    return v.iter().fold(1, |acc, &x| lcm(acc, x));
}

fn parse_instructions(source: &str) -> (Vec<usize>, NodeMap) {
    let (left, right) = source.split_once("\n\n").unwrap();

    let nodes = right.split('\n').map(|line| parse_line(line)).collect();

    (
        left.chars().map(|c| if c == 'L' { 0 } else { 1 }).collect(),
        nodes,
    )
}

fn parse_line(line: &str) -> (&str, (&str, &str)) {
    (&line[0..3], (&line[7..10], &line[12..15]))
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_sample_1() {
        let sample = read_input("8_sample_1");
        assert_eq!(exec(&sample).0, 2)
    }

    #[test]
    fn test_sample_2() {
        let sample = read_input("8_sample_2");
        assert_eq!(exec(&sample).0, 6)
    }

    #[test]
    fn test_parse_instructions() {
        let result = parse_instructions("LRLR\n\nAAA = (BBB, CCC)");
        assert_eq!(result.0, vec![0, 1, 0, 1]);
        assert_eq!(result.1, HashMap::from([("AAA", ("BBB", "CCC"))]));
    }
}
