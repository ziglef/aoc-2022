use std::{collections::HashSet, time::Instant};

fn main() {
    let start = Instant::now();

    let mut priorities: Vec<char> = Vec::new();
    let mut badges: Vec<char> = Vec::new();
    
    let rucksacks: Vec<&str> = include_str!("../input.txt").lines().collect();
    let end_input = Instant::now();
    
    // First problem
    for rucksack in &rucksacks {
        let compartments = rucksack.split_at(rucksack.len()/2);
        let mut items: HashSet<char> = compartments.0.chars().collect();
        let items2: HashSet<char> = compartments.1.chars().collect();
        
        items.retain(|item| items2.contains(item));
        priorities.push(*items.iter().last().unwrap());
    }
    println!(
        "A: {:?}",
        priorities.iter().map(|&p| if p.is_uppercase() { p as u32 - 'A' as u32 + 27 } else { p as u32 - 'a' as u32 + 1 }).sum::<u32>(),
    );
    let end_prob_a = Instant::now();

    // Second problem
    for group_rucksacks in rucksacks.chunks(3) {
        let mut items: HashSet<char> = group_rucksacks[0].chars().collect();
        let items2: HashSet<char> = group_rucksacks[1].chars().collect();
        let items3: HashSet<char> = group_rucksacks[2].chars().collect();
        
        items.retain(|item| items2.contains(item) && items3.contains(item));
        badges.push(*items.iter().last().unwrap());
    }
    println!(
        "B: {:?}",
        badges.iter().map(|&p| if p.is_uppercase() { p as u32 - 'A' as u32 + 27 } else { p as u32 - 'a' as u32 + 1 }).sum::<u32>(),
    );
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}