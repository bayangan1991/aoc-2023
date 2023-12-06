const INPUT_1: [(usize, usize); 4] = [(54, 239), (70, 1142), (82, 1295), (75, 1253)];

pub fn exec(_source: &str) -> (usize, usize) {
    (
        INPUT_1.iter().map(|(t, d)| math_solve(*t, *d)).product(),
        math_solve(54708275, 239114212951253),
    )
}

/*fn evaluate_options(time: usize, distance: usize) -> usize {
    let mut start = 0;
    let mut end = 0;
    for x in 0..time {
        if (time - x) * x > distance {
            start = x;
            break;
        }
    }

    for x in (0..time).rev() {
        if (time - x) * x > distance {
            end = x;
            break;
        }
    }

    if end != 0 {
        end - start + 1
    } else {
        time - start
    }
}*/

fn math_solve(time: usize, distance: usize) -> usize {
    let d = f64::sqrt((time * time - 4 * distance) as f64);

    let from = f64::floor((time as f64 - d) / 2_f64);
    let to = f64::ceil((time as f64 + d) / 2_f64);

    (to - from) as usize - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_options() {
        assert_eq!(math_solve(7, 9), 4);
        assert_eq!(math_solve(15, 40), 8);
        assert_eq!(math_solve(30, 200), 9);
    }
}
