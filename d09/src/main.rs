use std::{time::Instant, collections::HashSet};


fn move_tail(head_position: (i32, i32), tail_position: &mut (i32, i32)) -> () {
    if tail_position.0 != head_position.0 && tail_position.1 != head_position.1 {
        if tail_position.0.abs_diff(head_position.0) > 1 || tail_position.1.abs_diff(head_position.1) > 1 {
            if tail_position.0 < head_position.0 {
                tail_position.0 += 1;
            } else {
                tail_position.0 -= 1;
            }
        
            if tail_position.1 < head_position.1 {
                tail_position.1 += 1
            } else {
                tail_position.1 -= 1
            }
        }
    } else {
        // Check tail left
        if tail_position.0 == head_position.0 - 2 {
            tail_position.0 += 1
        }
    
        // Check tail right
        if tail_position.0 == head_position.0 + 2 {
            tail_position.0 -= 1
        }
    
        // Check tail up
        if tail_position.1 == head_position.1 + 2 {
            tail_position.1 -= 1
        }
    
        // Check tail down
        if tail_position.1 == head_position.1 - 2 {
            tail_position.1 += 1
        }
    }
}


fn main() {
    let start = Instant::now();

    let mut commands: Vec<(char, i32)> = Vec::new();

    include_str!("../input.txt").lines().for_each(|l| {
        let mut split = l.split_whitespace();
        commands.push((split.next().unwrap().parse::<char>().unwrap(), split.next().unwrap().parse::<i32>().unwrap()));
    });
    let end_input = Instant::now();

    // First problem
    let mut head_position: (i32, i32) = (0,0);
    let mut tail_position: (i32, i32) = (0,0);

    // tail as visited initial position to start with
    let mut visited_positions_a: HashSet<(i32, i32)> = HashSet::new();
    visited_positions_a.insert((0,0));
    for command in &commands {
        for _ in 0..command.1 {
            match command.0 {
                'L' => {
                    head_position.0 -= 1;
                },
                'R' => {
                    head_position.0 += 1;
                },
                'U' => {
                    head_position.1 += 1;
                },
                'D' => {
                    head_position.1 -= 1;
                },
                _ => {},
            }
            move_tail(head_position, &mut tail_position);
            visited_positions_a.insert(tail_position);
        }
    }
    println!(
        "A: {:?}",
        visited_positions_a.len(),
    );
    let end_prob_a = Instant::now();

    // Second problem
    const KNOT_AMOUNT: usize = 10;
    let mut rope_positions: Vec<(i32, i32)> = vec![(0,0); KNOT_AMOUNT];

    // tail as visited initial position to start with
    let mut visited_positions_b: HashSet<(i32, i32)> = HashSet::new();
    visited_positions_b.insert((0,0));
    for command in &commands {
        for _ in 0..command.1 {
            match command.0 {
                'L' => {
                    rope_positions[0].0 -= 1;
                },
                'R' => {
                    rope_positions[0].0 += 1;
                },
                'U' => {
                    rope_positions[0].1 += 1;
                },
                'D' => {
                    rope_positions[0].1 -= 1;
                },
                _ => {},
            }
            for i in 0..KNOT_AMOUNT-1 {
                move_tail(rope_positions[i], &mut rope_positions[i+1]);
            }
            visited_positions_b.insert(rope_positions[KNOT_AMOUNT-1]);
        }
    }
    println!(
        "B: {:?}",
        visited_positions_b.len(),
    );
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}