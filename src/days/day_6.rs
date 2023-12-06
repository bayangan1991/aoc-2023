const INPUT_1: [(usize, usize); 4] = [(54, 239), (70, 1142), (82, 1295), (75, 1253)];

pub fn exec(_source: &str) -> (usize, usize) {
    (
        INPUT_1
            .iter()
            .map(|(t, d)| evaluate_options(*t, *d))
            .product(),
        evaluate_options(54708275, 239114212951253),
    )
}

fn evaluate_options(time: usize, distance: usize) -> usize {
    let mut result = 0;
    for x in 0..time {
        if (time - x) * x > distance {
            result += 1
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_options() {
        assert_eq!(evaluate_options(7, 9), 4);
        assert_eq!(evaluate_options(15, 40), 8);
        assert_eq!(evaluate_options(30, 200), 9);
    }
}
