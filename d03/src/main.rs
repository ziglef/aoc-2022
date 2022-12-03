use std::collections::HashSet;

fn main() {
    let mut rucksacks: Vec<(&str, &str)> = Vec::new();
    let mut priorities: Vec<char> = Vec::new();
    let mut badges: Vec<char> = Vec::new();

    include_str!("../input.txt")
        .lines()
        .for_each(|i| {
            rucksacks.push(i.split_at(i.len()/2));

            let items: HashSet<char> = rucksacks.last().unwrap().0.chars().collect();
            for c in rucksacks.last().unwrap().1.chars() {
                if items.contains(&c) {
                    priorities.push(c);
                    break;
                }
            }
        });

    // First problem
    println!(
        "{}",
        priorities.iter().map(|&p| if p.is_uppercase() { p as u32 - 'A' as u32 + 27 } else { p as u32 - 'a' as u32 + 1 }).sum::<u32>(),
    );

    for group_rucksacks in rucksacks.chunks(3) {
        let items: HashSet<char> = format!("{}{}", group_rucksacks[0].0, group_rucksacks[0].1).chars().collect();
        let mut badge_found: bool = false;
        for c in format!("{}{}", group_rucksacks[1].0, group_rucksacks[1].1).chars() {
            if items.contains(&c) {
                for c2 in format!("{}{}", group_rucksacks[2].0, group_rucksacks[2].1).chars() {
                    if c2 == c && items.contains(&c2) {
                        badges.push(c);
                        badge_found = true;
                        break;
                    }
                }
                if badge_found {
                    break;
                }
            }
        }
    }

    // Second problem
    println!(
        "{}",
        badges.iter().map(|&p| if p.is_uppercase() { p as u32 - 'A' as u32 + 27 } else { p as u32 - 'a' as u32 + 1 }).sum::<u32>(),
    );
}