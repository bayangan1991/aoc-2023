mod days;
mod utils;

fn main() {
    let day_1 = utils::read_input("1");
    println!("Day 1, Part 1: {}", days::day_1::exec(&day_1, 1));
    println!("Day 1, Part 2: {}", days::day_1::exec(&day_1, 2));

    let day_2 = utils::read_input("2");
    println!("Day 2, Part 1: {}", days::day_2::exec(&day_2, 1));
    println!("Day 2, Part 2: {}", days::day_2::exec(&day_2, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_main() {
        main();
    }
}
