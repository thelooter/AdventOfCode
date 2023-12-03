use std::collections::HashMap;
use crate::read_lines;

fn main() {
    let mut digit_map = HashMap::new();
    digit_map.insert("one", 1);
    digit_map.insert("two", 2);
    digit_map.insert("three", 3);
    digit_map.insert("four", 4);
    digit_map.insert("five", 5);
    digit_map.insert("six", 6);
    digit_map.insert("seven", 7);
    digit_map.insert("eight", 8);
    digit_map.insert("nine", 9);

    //Read input
    let lines = read_lines("day-1/input.txt");

}