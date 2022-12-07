use std::{collections::HashSet, time::Instant};

fn main() {
    let start = Instant::now();
    
    let mut index = 4;
    let input: &str = include_str!("../input.txt").trim();
    let end_input = Instant::now();

    // First problem
    for window in input.chars().collect::<Vec<char>>().windows(4) {
        let hs = window.iter().collect::<HashSet<&char>>();
        if hs.len() == 4 {
            break;
        }
        index += 1;
    }
    println!(
        "A: {:?}",
        index,
    );
    let end_prob_a = Instant::now();

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
        "B: {:?}",
        index,
    );
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}