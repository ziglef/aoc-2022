use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut elfs: Vec<u32> = Vec::new();
    let mut elf_sum: u32 = 0;

    include_str!("../input.txt").lines().for_each(|i| {
        let amt: u32 = i.parse::<u32>().unwrap_or(0);

        if amt != 0 {
            elf_sum += amt;
        } else {
            elfs.push(elf_sum);
            elf_sum = 0;
        } 
    });
    let end_input = Instant::now();

    // First problem
    elfs.sort();
    println!(
        "A: {:?}",
        elfs.last().unwrap(),
    );
    let end_prob_a = Instant::now();

    // Second problem
    println!(
        "B: {:?}",
        elfs.iter().rev().take(3).sum::<u32>(),
    );
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}