use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut trees: Vec<Vec<u8>> = Vec::new();

    include_str!("../input.txt").lines().for_each(|l| {
        trees.push(l.chars().map(|c| c.to_digit(19).unwrap() as u8).collect::<Vec<u8>>());
    });
    let end_input = Instant::now();

    // First problem
    let mut visible_count = trees.len() * 4 - 4;
    let mut tree_house_options: Vec<(usize, usize)> = Vec::new();
    for row in 1..trees.len()-1 {
        for column in 1..trees.get(row).unwrap().len()-1 {
            let mut visible;
            let tree = trees.get(row).unwrap().get(column).unwrap();

            // Test Left
            visible = true;
            for other_column in (0..column).rev() {
                if other_column == column {
                    continue;
                }
                if trees.get(row).unwrap().get(other_column).unwrap() >= tree {
                    visible = false;
                    break;
                }
            }
            if visible {
                visible_count += 1;
                tree_house_options.push((row, column));
                continue;
            }

            // Test Right
            visible = true;
            for other_column in column+1..trees.get(row).unwrap().len() {
                if other_column == column {
                    continue;
                }
                if trees.get(row).unwrap().get(other_column).unwrap() >= tree {
                    visible = false;
                    break;
                }
            }
            if visible {
                visible_count += 1;
                tree_house_options.push((row, column));
                continue;
            }

            // Test Top
            visible = true;
            for other_row in (0..row).rev() {
                if other_row == row {
                    continue;
                }
                if trees.get(other_row).unwrap().get(column).unwrap() >= tree {
                    visible = false;
                    break;
                }
            }
            if visible {
                visible_count += 1;
                tree_house_options.push((row, column));
                continue;
            }

            // Test Bottom
            visible = true;
            for other_row in row..trees.len() {
                if other_row == row {
                    continue;
                }
                if trees.get(other_row).unwrap().get(column).unwrap() >= tree {
                    visible = false;
                    break;
                }
            }
            if visible {
                visible_count += 1;
                tree_house_options.push((row, column));
            }
        }
    }
    println!(
        "A: {:?}",
        visible_count,
    );
    let end_prob_a = Instant::now();

    // Second problem
    let mut max_scenic_score: u32 = 0;
    for (row, column) in tree_house_options {
        let mut scenic_score: u32 = 1;
        let mut visible_trees: u32 = 0;
        let tree = trees.get(row).unwrap().get(column).unwrap();

        // Test Left
        for other_column in (0..column).rev() {
            if other_column == column {
                continue;
            }
            visible_trees += 1;
            if trees.get(row).unwrap().get(other_column).unwrap() >= tree {
                break;
            }
        }
        if visible_trees == 0 {
            break;
        } else {
            scenic_score *= visible_trees;
            visible_trees = 0;
        }

        // Test Right
        for other_column in column+1..trees.get(row).unwrap().len() {
            if other_column == column {
                continue;
            }
            visible_trees += 1;
            if trees.get(row).unwrap().get(other_column).unwrap() >= tree {
                break;
            }
        }
        if visible_trees == 0 {
            break;
        } else {
            scenic_score *= visible_trees;
            visible_trees = 0;
        }

        // Test Top
        for other_row in (0..row).rev() {
            if other_row == row {
                continue;
            }
            visible_trees += 1;
            if trees.get(other_row).unwrap().get(column).unwrap() >= tree {
                break;
            }
        }
        if visible_trees == 0 {
            break;
        } else {
            scenic_score *= visible_trees;
            visible_trees = 0;
        }

        // Test Bottom
        for other_row in row..trees.len() {
            if other_row == row {
                continue;
            }
            visible_trees += 1;
            if trees.get(other_row).unwrap().get(column).unwrap() >= tree {
                break;
            }
        }
        if visible_trees == 0 {
            break;
        } else {
            scenic_score *= visible_trees;
        }
        if scenic_score > max_scenic_score {
            max_scenic_score = scenic_score;
        }
    }
    println!(
        "B: {:?}",
        max_scenic_score,
    );
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}