use std::collections::{HashMap, HashSet, VecDeque};

pub fn exec(source: &str) -> (usize, usize) {
    let mut start: Option<Pipe> = None;
    let mut pipes = vec![];

    for (y, line) in source.split('\n').enumerate() {
        let (new_pipes, new_start) = parse_line(&line, y as isize);

        if Option::is_some(&new_start) {
            start = new_start;
        }

        pipes.extend(new_pipes);
    }

    let map = pipes
        .iter()
        .map(|&pipe| ((pipe.x, pipe.y), pipe))
        .collect::<HashMap<_, _>>();

    let mut main_loop: Option<HashSet<Pipe>> = None;

    let part_a = match start {
        None => 0,
        Some(s) => {
            let result = get_max_pipe_distance(&s, &map);
            main_loop = Some(result.1);
            result.0
        }
    };

    let main_loop = main_loop.unwrap();
    let loop_map: HashMap<_, _> = main_loop
        .iter()
        .map(|pipe| ((pipe.x, pipe.y), pipe.clone()))
        .collect();

    let size_y = source.matches('\n').count() as isize;
    let size_x = source.find('\n').unwrap() as isize;

    let mut inside = 0;

    for y in 0..=size_y {
        let mut north_on = false;
        let mut south_on = false;
        for x in 0..=size_x {
            match loop_map.get(&(x, y)) {
                None => {
                    if north_on && south_on {
                        inside += 1
                    };
                }
                Some(pipe) => {
                    if pipe.n {
                        north_on = !north_on;
                    };
                    if pipe.s {
                        south_on = !south_on;
                    };
                }
            }
        }
    }

    (part_a, inside)
}

fn get_max_pipe_distance(
    start: &Pipe,
    map: &HashMap<(isize, isize), Pipe>,
) -> (usize, HashSet<Pipe>) {
    let mut seen = HashSet::from([start.clone()]);

    let mut stack = VecDeque::from([start.clone()]);

    while let Some(current) = stack.pop_front() {
        let neighbours = current.get_connections(&map);

        neighbours.iter().for_each(|pipe| {
            if !seen.contains(pipe) {
                seen.insert(pipe.clone());
                stack.push_front(pipe.clone());
            }
        });
    }

    (seen.len() / 2, seen)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Pipe {
    char: char,
    start: bool,
    x: isize,
    y: isize,
    n: bool,
    e: bool,
    s: bool,
    w: bool,
}

impl Pipe {
    fn get_connections(&self, map: &HashMap<(isize, isize), Pipe>) -> HashSet<Pipe> {
        let mut result = HashSet::with_capacity(2);

        for (d, (x, y)) in CARDINALS {
            let target = (self.x + x, self.y + y);

            match map.get(&target) {
                None => {}
                Some(neighbour) => match d {
                    Cardinal::N => {
                        if neighbour.s && (self.start || self.n) {
                            result.insert(neighbour.clone());
                        };
                    }
                    Cardinal::S => {
                        if neighbour.n && (self.start || self.s) {
                            result.insert(neighbour.clone());
                        };
                    }
                    Cardinal::E => {
                        if neighbour.w && (self.start || self.e) {
                            result.insert(neighbour.clone());
                        };
                    }
                    Cardinal::W => {
                        if neighbour.e && (self.start || self.w) {
                            result.insert(neighbour.clone());
                        };
                    }
                },
            };
        }

        result
    }
}

enum Cardinal {
    N,
    S,
    E,
    W,
}

const CARDINALS: [(Cardinal, (isize, isize)); 4] = [
    (Cardinal::N, (0, -1)),
    (Cardinal::S, (0, 1)),
    (Cardinal::W, (-1, 0)),
    (Cardinal::E, (1, 0)),
];

const NORTH: [char; 3] = ['|', 'L', 'J'];
const EAST: [char; 3] = ['-', 'F', 'L'];
const SOUTH: [char; 3] = ['|', '7', 'F'];
const WEST: [char; 3] = ['-', 'J', '7'];

fn parse_line(line: &str, y: isize) -> (Vec<Pipe>, Option<Pipe>) {
    let mut start = None;

    let result = line
        .chars()
        .enumerate()
        .map(|(x, c)| {
            let new = Pipe {
                char: c,
                start: c == 'S',
                x: x as isize,
                y,
                n: NORTH.contains(&c),
                e: EAST.contains(&c),
                s: SOUTH.contains(&c),
                w: WEST.contains(&c),
            };
            if new.start {
                start = Some(new)
            }
            new
        })
        .collect();

    (result, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("F", 0),
            (
                vec![Pipe {
                    char: 'F',
                    start: false,
                    x: 0,
                    y: 0,
                    n: false,
                    e: true,
                    s: true,
                    w: false,
                }],
                None
            )
        )
    }

    #[test]
    fn test_map_connections() {
        let pipe_1 = Pipe {
            char: 'x',
            start: false,
            x: 0,
            y: 0,
            n: false,
            s: false,
            e: true,
            w: true,
        };
        let pipe_2 = Pipe {
            char: 'x',
            start: false,
            x: 1,
            y: 0,
            n: false,
            s: false,
            e: true,
            w: true,
        };
        let pipe_3 = Pipe {
            char: 'x',
            start: false,
            x: 2,
            y: 0,
            n: false,
            s: false,
            e: true,
            w: true,
        };
        let pipe_4 = Pipe {
            char: 'x',
            start: false,
            x: 1,
            y: 1,
            n: true,
            s: false,
            e: true,
            w: false,
        };
        let map = HashMap::from([
            ((0, 0), pipe_1),
            ((1, 0), pipe_2),
            ((2, 0), pipe_3),
            ((1, 1), pipe_4),
        ]);

        assert_eq!(
            pipe_2.get_connections(&map),
            HashSet::from([pipe_1, pipe_3])
        )
    }

    #[test]
    fn test_sample_1() {
        let sample = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        assert_eq!(exec(&sample).0, 4);
    }

    #[test]
    fn test_sample_2() {
        let sample = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        assert_eq!(exec(&sample).0, 8);
    }

    #[test]
    fn test_sample_3() {
        let sample = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(exec(&sample).1, 4);
    }
}
