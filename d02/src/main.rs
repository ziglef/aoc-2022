use std::collections::HashMap;

fn main() {
    let total_score: HashMap<(char, char), u32> = HashMap::from(
        [
            // (their shape, your shape), pts for your shape + pts for result
            (('A', 'X'), 1+3),
            (('A', 'Y'), 2+6),
            (('A', 'Z'), 3+0),
            (('B', 'X'), 1+0),
            (('B', 'Y'), 2+3),
            (('B', 'Z'), 3+6),
            (('C', 'X'), 1+6),
            (('C', 'Y'), 2+0),
            (('C', 'Z'), 3+3),
        ]
    );
    let play_to_make: HashMap<(char, char), char> = HashMap::from(
        [
            // (their shape, result you should have), shape to play
            (('A', 'X'), 'Z'),
            (('A', 'Y'), 'X'),
            (('A', 'Z'), 'Y'),
            (('B', 'X'), 'X'),
            (('B', 'Y'), 'Y'),
            (('B', 'Z'), 'Z'),
            (('C', 'X'), 'Y'),
            (('C', 'Y'), 'Z'),
            (('C', 'Z'), 'X'),
        ]
    );
    let mut plays: Vec<(char, char)> = Vec::new();

    include_str!("../input.txt")
        .lines()
        .for_each(|i| {
            let mut split = i.split_whitespace();
            plays.push((split.next().unwrap().parse::<char>().unwrap(), split.next().unwrap().parse::<char>().unwrap()));
        });

    // First problem
    println!(
        "{}",
        plays.iter().map(|k| total_score.get(k).unwrap()).sum::<u32>(),
    );

    // Second problem
    println!(
        "{}",
        plays.iter().map(|k| total_score.get(&(k.0, *play_to_make.get(k).unwrap())).unwrap()).sum::<u32>(),
    );
}