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

    let day_6 = utils::read_input("6");
    let day_6 = days::day_6::exec(&day_6);
    println!("Day 6, Part 1: {}", day_6.0);
    println!("Day 6, Part 2: {}\n", day_6.1);

    let day_7 = utils::read_input("7");
    let day_7 = days::day_7::exec(&day_7);
    println!("Day 7, Part 1: {}", day_7.0);
    println!("Day 7, Part 2: {}\n", day_7.1);

    let day_8 = utils::read_input("8");
    let day_8 = days::day_8::exec(&day_8);
    println!("Day 8, Part 1: {}", day_8.0);
    println!("Day 8, Part 2: {}\n", day_8.1);

    let day_9 = utils::read_input("9");
    let day_9 = days::day_9::exec(&day_9);
    println!("Day 9, Part 1: {}", day_9.0);
    println!("Day 9, Part 2: {}\n", day_9.1);

    let day_10 = utils::read_input("10");
    let day_10 = days::day_10::exec(&day_10);
    println!("Day 10, Part 1: {}", day_10.0);
    println!("Day 10, Part 2: {}\n", day_10.1);

    let day_11 = utils::read_input("11");
    let day_11 = days::day_11::exec(&day_11);
    println!("Day 11, Part 1: {}", day_11.0);
    println!("Day 11, Part 2: {}\n", day_11.1);

    let day_12 = utils::read_input("12");
    let day_12 = days::day_12::exec(&day_12);
    println!("Day 12, Part 1: {}", day_12.0);
    println!("Day 12, Part 2: {}\n", day_12.1);

    let day_13 = utils::read_input("13");
    let day_13 = days::day_13::exec(&day_13);
    println!("Day 13, Part 1: {}", day_13.0);
    println!("Day 13, Part 2: {}\n", day_13.1);
}
