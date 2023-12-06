mod days;
mod utils;

fn main() {
    let day_1 = utils::read_input("1");
    let day_1 = days::day_1::exec(&day_1);
    println!("Day 1, Part 1: {}", day_1.0);
    println!("Day 1, Part 2: {}\n", day_1.1);

    let day_2 = utils::read_input("2");
    let day_2 = days::day_2::exec(&day_2);
    println!("Day 2, Part 1: {}", day_2.0);
    println!("Day 2, Part 2: {}\n", day_2.1);

    let day_3 = utils::read_input("3");
    let day_3 = days::day_3::exec(&day_3);
    println!("Day 3, Part 1: {}", day_3.0);
    println!("Day 3, Part 2: {}\n", day_3.1);

    let day_4 = utils::read_input("4");
    let day_4 = days::day_4::exec(&day_4);
    println!("Day 4, Part 1: {}", day_4.0);
    println!("Day 4, Part 2: {}\n", day_4.1);

    let day_5 = utils::read_input("5");
    let day_5 = days::day_5::exec(&day_5);
    println!("Day 5, Part 1: {}", day_5.0);
    println!("Day 5, Part 2: {}\n", day_5.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_main() {
        main();
    }
}
