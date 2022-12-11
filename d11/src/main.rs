use std::time::Instant;


#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operator: char,
    operation_magnitude: usize, // 0 means to use the value of the item being operated on
    test_by: usize,
    test_true: usize,
    test_false: usize
}

fn main() {
    let start = Instant::now();

    let mut monkeys: Vec<Monkey> = Vec::new();
    include_str!("../input.txt").lines().collect::<Vec<&str>>().chunks(7).for_each(|c| {
        // I don't feel like dealing with Option right now, so pushing a "default" struct
        monkeys.push(Monkey {items: Vec::new(), operator: '+', operation_magnitude: 0, test_by: 0, test_true: 0, test_false: 0});

        // items
        for item in c[1].split_whitespace().skip(2).collect::<Vec<&str>>().iter().map(|s| s.replace(",", "")).collect::<Vec<String>>() {
            monkeys.last_mut().unwrap().items.push(item.parse::<usize>().unwrap());
        }
        
        // operator
        monkeys.last_mut().unwrap().operator = c[2].split_whitespace().skip(4).next().unwrap().parse::<char>().unwrap();
        
        // operation magnitude
        monkeys.last_mut().unwrap().operation_magnitude = c[2].split_whitespace().last().unwrap().parse::<usize>().unwrap_or(0);

        // test_by
        monkeys.last_mut().unwrap().test_by = c[3].split_whitespace().last().unwrap().parse::<usize>().unwrap();

        // test_true
        monkeys.last_mut().unwrap().test_true = c[4].split_whitespace().last().unwrap().parse::<usize>().unwrap();

        // test_false
        monkeys.last_mut().unwrap().test_false = c[5].split_whitespace().last().unwrap().parse::<usize>().unwrap();
    });
    let end_input = Instant::now();

    // First problem
    let mut inspections_a: Vec<usize> = vec![0; monkeys.len()];
    let mut monkeys_a = monkeys.clone();
    for _ in 0..20 {
        for monkey_index in 0..monkeys_a.len() {
            let monkey = monkeys_a.get(monkey_index).unwrap().clone();
            inspections_a[monkey_index] += monkey.items.len();
            for item_index in 0..monkey.items.len() {
                let old_item = monkey.items[item_index];
                let mut new_item = old_item;
                // Inspection
                if monkey.operator == '+' {
                    if monkey.operation_magnitude == 0 {
                        new_item += old_item;
                    } else {
                        new_item += monkey.operation_magnitude;
                    }
                // '*'
                } else {
                    if monkey.operation_magnitude == 0 {
                        new_item *= old_item;
                    } else {
                        new_item *= monkey.operation_magnitude;
                    }
                }
                new_item /= 3;
                // Throwing
                if new_item % monkey.test_by == 0 {
                    monkeys_a[monkey.test_true].items.push(new_item);
                    monkeys_a[monkey_index].items.remove(0);
                } else {
                    monkeys_a[monkey.test_false].items.push(new_item);
                    monkeys_a[monkey_index].items.remove(0);
                }
            }
        }
    }
    inspections_a.sort();
    println!(
        "A: {:?}",
        inspections_a[inspections_a.len()-1] * inspections_a[inspections_a.len()-2],
    );
    let end_prob_a = Instant::now();

    // Second problem
    let mut inspections_b: Vec<usize> = vec![0; monkeys.len()];
    let mut monkeys_b = monkeys.clone();
    let common_div: usize = monkeys_b.iter().map(|m| m.test_by).product();
    for _ in 0..10000 {
        for monkey_index in 0..monkeys_b.len() {
            let monkey = monkeys_b.get(monkey_index).unwrap().clone();
            inspections_b[monkey_index] += monkey.items.len();
            for item_index in 0..monkey.items.len() {
                let old_item = monkey.items[item_index];
                let mut new_item = old_item;
                // Inspection
                if monkey.operator == '+' {
                    if monkey.operation_magnitude == 0 {
                        new_item += old_item;
                    } else {
                        new_item += monkey.operation_magnitude;
                    }
                // '*'
                } else {
                    if monkey.operation_magnitude == 0 {
                        new_item *= old_item;
                    } else {
                        new_item *= monkey.operation_magnitude;
                    }
                }
                new_item %= common_div;
                // Throwing
                if new_item % monkey.test_by == 0 {
                    monkeys_b[monkey.test_true].items.push(new_item);
                    monkeys_b[monkey_index].items.remove(0);
                } else {
                    monkeys_b[monkey.test_false].items.push(new_item);
                    monkeys_b[monkey_index].items.remove(0);
                }
            }
        }
    }
    inspections_b.sort();
    println!(
        "B: {:?}",
        inspections_b[inspections_b.len()-1] * inspections_b[inspections_b.len()-2],
    );
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}