use day_01::process_part_1;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read file");
    println!("{}", process_part_1(&input));
}
