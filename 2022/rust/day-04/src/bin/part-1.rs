use day_04::process_part_1;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("file not found");
    println!("{}", process_part_1(&input));
}
