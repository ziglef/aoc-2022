use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("../input.txt").trim();
    let mut index = 4;

    // First problem
    for window in input.chars().collect::<Vec<char>>().windows(4) {
        let hs = window.iter().collect::<HashSet<&char>>();
        if hs.len() == 4 {
            break;
        }
        index += 1;
    }

    println!(
        "{}",
        index,
    );

    // Second problem
    index = 14;
    for window in input.chars().collect::<Vec<char>>().windows(14) {
        let hs = window.iter().collect::<HashSet<&char>>();
        if hs.len() == 14 {
            break;
        }
        index += 1;
    }
    println!(
        "{}",
        index,
    );
}