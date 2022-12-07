use day_01::*;
use std::fs;

fn main () {
    let file = fs::read_to_string("./test.txt").unwrap();
    println!("{}", process_part2(&file));
}