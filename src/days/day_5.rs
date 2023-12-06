#[derive(Clone, PartialEq, Debug)]
struct Map {
    range: Range,
    offset: isize,
}

impl Map {
    fn offset(&self, value: usize) -> usize {
        return (value as isize + self.offset) as usize;
    }
}

type Range = (usize, usize);
type MapVec = Vec<Vec<Map>>;

pub fn exec(source: &str) -> Range {
    let (seeds, maps) = parse_input(source);

    let part_1_seeds: Vec<Range> = seeds.iter().map(|seed| (*seed, *seed)).collect();

    let mut part_1 = vec![];

    part_1_seeds.iter().for_each(|seed| {
        part_1.extend(process_maps(*seed, &maps));
    });

    let part_2_seeds: Vec<Range> = seeds
        .chunks(2)
        .map(|item| {
            (
                *item.get(0).unwrap(),
                *item.get(0).unwrap() + *item.get(1).unwrap() - 1,
            )
        })
        .collect();

    let mut part_2 = vec![];

    part_2_seeds.iter().for_each(|seed| {
        part_2.extend(process_maps(*seed, &maps));
    });

    (
        part_1.iter().map(|thing| thing.0).min().unwrap(),
        part_2.iter().map(|thing| thing.0).min().unwrap(),
    )
}

fn parse_input(source: &str) -> (Vec<usize>, MapVec) {
    let mut seeds: Vec<usize> = vec![];
    let mut maps: MapVec = vec![];
    let mut current_map = vec![];

    for line in source.split('\n') {
        if line.starts_with("seeds: ") {
            let (_, seeds_str) = line.split_once(": ").unwrap();
            seeds_str
                .split(" ")
                .for_each(|seed| seeds.push(seed.parse().unwrap()));
        } else if line.ends_with(" map:") {
            if current_map.iter().count() > 0 {
                maps.push(current_map.clone());
                current_map = vec![];
            }
        } else if line.len() > 0 {
            let mut parts = line.split(' ');
            let finish: usize = parts.next().unwrap().parse().unwrap();
            let start: usize = parts.next().unwrap().parse().unwrap();
            let range: usize = parts.next().unwrap().parse().unwrap();

            current_map.push(Map {
                range: (start, start + range),
                offset: finish as isize - start as isize,
            })
        }
    }

    if current_map.iter().count() > 0 {
        maps.push(current_map.clone());
    }

    (seeds, maps)
}

fn process_maps(seed: Range, maps: &MapVec) -> Vec<Range> {
    let mut to_work = vec![seed];
    let mut result = vec![];

    for map in maps {
        for work in &to_work {
            result.extend(process_map(*work, &map));
        }
        to_work = result.clone();
        result = vec![];
    }

    to_work
}

fn process_map(seed: Range, maps: &Vec<Map>) -> Vec<Range> {
    let mut devoured = false;
    let mut chomped = vec![];

    let (mut min, mut max) = seed;

    // 4 scenarios per chomp
    // |-----<==============>------| Seed Range
    // |--|===========|------------| 1
    // |--|===================|----| 2
    // |-------------|========|----| 3
    // |--------|======|-----------| 4

    for map in maps {
        if min >= map.range.0 && min <= map.range.1 {
            if max > map.range.1 {
                // Scenario 1
                chomped.push((map.offset(min), map.offset(map.range.1)));
                if max + 1 == map.range.1 {
                    devoured = true;
                    break;
                }
                min = map.range.1;
            } else {
                // Scenario 2
                chomped.push((map.offset(min), map.offset(max)));
                devoured = true;
                // Completely swallowed -- break
                break;
            }
        } else if max >= map.range.0 && max < map.range.1 {
            // Scenario 3
            if min <= map.range.0 {
                chomped.push((map.offset(map.range.0), map.offset(max)));
                if min == map.range.0 {
                    devoured = true;
                    break;
                }
                max = map.range.0 - 1;
            } else {
                // Scenario 4
                chomped.push((map.offset(map.range.0), map.offset(map.range.1)));
                // Splits range into 2 distinct parts
                // Check each
                chomped.extend(process_map((min, map.range.0 - 1), &maps));
                chomped.extend(process_map((max + 1, map.range.1), &maps));
            }
        }
    }

    if !devoured {
        chomped.push((min, max))
    }

    chomped
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    #[test]
    fn test_parse_maps() {
        let (seeds, maps) = parse_input(&read_input("5_sample_1"));

        assert_eq!(seeds, vec![79, 14, 55, 13]);

        let seed_to_soil = maps.get(0).unwrap().clone();
        assert_eq!(
            seed_to_soil,
            vec![
                Map {
                    range: (98, 100),
                    offset: -48,
                },
                Map {
                    range: (50, 98),
                    offset: 2,
                },
            ]
        );
    }

    #[test]
    fn test_walk_map_single_number() {
        let maps = vec![
            Map {
                range: (98, 99),
                offset: -48,
            },
            Map {
                range: (50, 97),
                offset: 2,
            },
        ];

        // Seed Ranges
        let result = process_map((79, 79), &maps);
        assert_eq!(result, vec![(81, 81)]);
        let result = process_map((14, 14), &maps);
        assert_eq!(result, vec![(14, 14)]);
        let result = process_map((55, 55), &maps);
        assert_eq!(result, vec![(57, 57)]);
        let result = process_map((13, 13), &maps);
        assert_eq!(result, vec![(13, 13)]);
        // Edge of range
        let result = process_map((99, 99), &maps);
        assert_eq!(result, vec![(51, 51)]);
        let result = process_map((97, 97), &maps);
        assert_eq!(result, vec![(99, 99)]);
    }

    #[test]
    fn test_each_seed_in_sample() {
        let sample = read_input("5_sample_1");
        let (_, maps) = parse_input(&sample);

        let results = [13, 52, 41, 34, 34, 35, 35];
        let mut seed = (13, 13);

        for (index, map) in maps.iter().enumerate() {
            seed = *process_map(seed, map).iter().next().unwrap();
            assert_eq!(seed.0, *results.get(index).unwrap());
            assert_eq!(seed.1, *results.get(index).unwrap());
        }
    }

    #[test]
    fn test_example() {
        let sample = String::from("seeds: 1 10\n\nseed-to-location map:\n10 1 1");

        assert_eq!(exec(&sample).0, 10)
    }

    #[test]
    fn test_sample_1() {
        let sample = read_input("5_sample_1");
        assert_eq!(exec(&sample).0, 35)
    }

    #[test]
    fn test_sample_2() {
        let sample = read_input("5_sample_1");
        assert_eq!(exec(&sample).1, 46)
    }

    #[test]
    fn test_paul_sample() {
        let sample = String::from(
            "seeds: 0 50

seed-to-soil map:
25 0 25
50 25 50

soil-to-fertilizer map:
0 0 0

fertilizer-to-water map:
0 0 0

water-to-light map:
0 0 0

light-to-temperature map:
0 0 0

temperature-to-humidity map:
0 0 0

humidity-to-location map:
0 74 1",
        );

        assert_eq!(exec(&sample).1, 0)
    }

    #[test]
    fn test_ryan_sample() {
        let sample = String::from(
            "seeds: 7 1

seed-to-soil map:
6 7 1

soil-to-fertilizer map:
5 6 1

fertilizer-to-water map:
4 5 1

water-to-light map:
3 4 1

light-to-temperature map:
2 3 1

temperature-to-humidity map:
1 2 1

humidity-to-location map:
0 1 1",
        );

        assert_eq!(exec(&sample).1, 0)
    }
}
