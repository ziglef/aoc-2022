use std::collections::HashSet;

fn main() {
    let rucksacks: Vec<&str> = include_str!("../input.txt").lines().collect();
    let mut priorities: Vec<char> = Vec::new();
    let mut badges: Vec<char> = Vec::new();
    
    // First problem
    for rucksack in &rucksacks {
        let compartments = rucksack.split_at(rucksack.len()/2);
        let mut items: HashSet<char> = compartments.0.chars().collect();
        let items2: HashSet<char> = compartments.1.chars().collect();
        
        items.retain(|item| items2.contains(item));
        priorities.push(*items.iter().last().unwrap());
    }
    println!(
        "{}",
        priorities.iter().map(|&p| if p.is_uppercase() { p as u32 - 'A' as u32 + 27 } else { p as u32 - 'a' as u32 + 1 }).sum::<u32>(),
    );

    for group_rucksacks in rucksacks.chunks(3) {
        let mut items: HashSet<char> = group_rucksacks[0].chars().collect();
        let items2: HashSet<char> = group_rucksacks[1].chars().collect();
        let items3: HashSet<char> = group_rucksacks[2].chars().collect();
        
        items.retain(|item| items2.contains(item) && items3.contains(item));
        badges.push(*items.iter().last().unwrap());
    }

    // Second problem
    println!(
        "{}",
        badges.iter().map(|&p| if p.is_uppercase() { p as u32 - 'A' as u32 + 27 } else { p as u32 - 'a' as u32 + 1 }).sum::<u32>(),
    );
}