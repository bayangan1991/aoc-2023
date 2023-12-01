mod utils;
mod days;

fn main() {
    let day_1 = utils::read_input("1");
    println!("Day 1, Part 1: {}", days::day_1::exec(&day_1, 1));
    println!("Day 1, Part 2: {}", days::day_1::exec(&day_1, 2));
}
