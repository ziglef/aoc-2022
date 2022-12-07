use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut elf_pairs: Vec<((u32, u32), (u32, u32))> = Vec::new();
    
    include_str!("../input.txt").lines().for_each(|l| {
        let mut l_split = l.split(',');
        let mut pair_1 = l_split.next().unwrap().split("-");
        let mut pair_2 = l_split.next().unwrap().split("-");
        
        elf_pairs.push(
            (
                (pair_1.next().unwrap().parse::<u32>().unwrap(), pair_1.next().unwrap().parse::<u32>().unwrap()),
                (pair_2.next().unwrap().parse::<u32>().unwrap(), pair_2.next().unwrap().parse::<u32>().unwrap())
            )
        )
    });
    let end_input = Instant::now();
    
    // First problem
    println!(
        "A: {:?}",
        elf_pairs.iter().filter(|pair| {
            (pair.1.0 >= pair.0.0 && pair.1.1 <= pair.0.1) ||
            (pair.0.0 >= pair.1.0 && pair.0.1 <= pair.1.1)
        }).count(),
    );
    let end_prob_a = Instant::now();

    // Second problem
    println!(
        "B: {:?}",
        elf_pairs.iter().filter(|pair| {
            (pair.0.0 >= pair.1.0 && pair.0.0 <= pair.1.1) ||
            (pair.0.1 >= pair.1.0 && pair.0.1 <= pair.1.1) ||
            (pair.1.0 >= pair.0.0 && pair.1.0 <= pair.0.1) ||
            (pair.1.1 >= pair.0.0 && pair.1.1 <= pair.0.1)
        }).count(),
    );
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}