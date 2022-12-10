use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut commands: Vec<(&str, i32)> = Vec::new();
    include_str!("../input.txt").lines().for_each(|l| {
        let mut parts = l.split_whitespace();
        commands.push((parts.next().unwrap(), parts.next().unwrap_or("0").parse::<i32>().unwrap_or(0)));
    });
    let end_input = Instant::now();

    // First problem
    let mut sum: i32 = 0;
    let mut x: i32 = 1;
    let mut cycle: i32 = 1;
    let mut cycle_to_check: i32 = 20;

    for command in &commands {
        match command.0 {
            "noop" => {
                cycle += 1;
            },
            "addx" => {
                cycle += 2;
                x += command.1;
            }
            _ => {}
        }
        if cycle == cycle_to_check {
            sum += cycle_to_check * x;
            cycle_to_check += 40;
        } else if cycle == cycle_to_check + 1 {
            sum += cycle_to_check * (x - command.1);
            cycle_to_check += 40;
        }
    }
    println!(
        "A: {:?}",
        sum,
    );
    let end_prob_a = Instant::now();

    // Second problem
    let mut screen: Vec<char> = vec!['.'; 240];
    let mut crt_pos: usize = 0;
    x = 1;

    for command in commands {
        if (crt_pos % 40) as i32 >= x - 1 && (crt_pos % 40) as i32 <= x + 1 {
            screen[crt_pos] = '#';
        }
        crt_pos += 1;
        match command.0 {
            "noop" => {},
            "addx" => {
                if (crt_pos % 40) as i32 >= x - 1 && (crt_pos % 40) as i32 <= x + 1 {
                    screen[crt_pos] = '#';
                }
                crt_pos += 1;
                x += command.1;
            }
            _ => {}
        }
    }
    println!(
        "B:",
    );
    for c in screen.chunks(40) {
        println!("{:?}", c);
    }
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}