use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut crate_stacks: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<(usize, usize, usize)> = Vec::new();
    let mut current_stack: usize = 1;
    let mut parsing_stacks: bool = true;

    include_str!("../input.txt").lines().for_each(|l| {
        if parsing_stacks {
            for elf_crate in l.chars().collect::<Vec<char>>().chunks(4) {
                if elf_crate[0] == '[' {
                    if crate_stacks.len() < current_stack {
                        crate_stacks.push(Vec::new());
                    }
                    crate_stacks.get_mut(current_stack-1).unwrap().insert(0, elf_crate[1]);
                    current_stack += 1;
                } else if elf_crate[1] == '1' {
                    parsing_stacks = false;
                    break;
                } else {
                    if crate_stacks.len() < current_stack {
                        crate_stacks.push(Vec::new());
                    }
                    current_stack += 1;
                    continue;
                }
            }
            current_stack = 1;
        } else {
            if l.len() > 0 {
                let fragments = l.split_whitespace().collect::<Vec<&str>>();
                movements.push(
                    (
                        fragments.get(1).unwrap().parse::<usize>().unwrap(),
                        fragments.get(3).unwrap().parse::<usize>().unwrap()-1,
                        fragments.get(5).unwrap().parse::<usize>().unwrap()-1
                    )
                );
            }
        }
    });
    let mut crate_stacks_2: Vec<Vec<char>> = crate_stacks.clone();
    let end_input = Instant::now();

    // First problem
    for movement in &movements {
        for _ in 0..movement.0 {
            let _crate = crate_stacks.get_mut(movement.1).unwrap().pop().unwrap();
            crate_stacks.get_mut(movement.2).unwrap().push(_crate);
        }
    }
    println!(
        "A: {:?}",
        crate_stacks.iter().map(|c| c.last().unwrap()).collect::<Vec<&char>>(),
    );
    let end_prob_a = Instant::now();
    
    // Second problem
    
    for movement in &movements {
        let crate_stacks_2_len = crate_stacks_2.get(movement.2).unwrap().len();
        for _ in 0..movement.0 {
            let _crate = crate_stacks_2.get_mut(movement.1).unwrap().pop().unwrap();
            crate_stacks_2.get_mut(movement.2).unwrap().insert(crate_stacks_2_len, _crate);
        }
    }
    println!(
        "B: {:?}",
        crate_stacks_2.iter().map(|c| c.last().unwrap()).collect::<Vec<&char>>(),
    );
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}