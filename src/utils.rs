use std::fs;


pub fn read_input(day: &str) -> String {
    let filepath = format!("data/{}", day);
    let err = format!("Unable to find '{filepath}'");
    fs::read_to_string(filepath).expect(&err)
}