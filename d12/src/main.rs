use std::time::Instant;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: (usize, usize),
    cost: usize,
}

fn neighbours(pos: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbours_= Vec::new();

    // Right
    if pos.0 < height - 1 {
        neighbours_.push((pos.0 + 1, pos.1));
    }
    // Left
    if pos.0 > 0 {
        neighbours_.push((pos.0 - 1, pos.1));
    }
    // Down
    if pos.1 < width - 1 {
        neighbours_.push((pos.0, pos.1 + 1));
    }
    // Up
    if pos.1 > 0 {
        neighbours_.push((pos.0, pos.1 - 1));
    }

    neighbours_
}

fn shortest_path(adj_list: &HashMap<(usize, usize), Vec<Edge>>, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();

    for key in adj_list.keys() {
        dist.insert(*key, usize::MAX);
    }
    *dist.get_mut(&start).unwrap() = 0usize;

    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return Some(cost); }

        if cost > *dist.get(&position).unwrap() { continue; }

        for edge in adj_list.get(&position).unwrap() {
            let next = State { cost: cost + edge.cost, position: edge.node };

            if next.cost < *dist.get(&next.position).unwrap() {
                heap.push(next);
                *dist.get_mut(&next.position).unwrap() = next.cost;
            }
        }
    }

    None
}

fn main() {
    let start = Instant::now();

    let mut line: usize = 0;
    let mut start_pos = (0,0);
    let mut end_pos = (0,0);
    let mut hills: Vec<Vec<u32>> = Vec::new();
    include_str!("../input.txt").lines().for_each(|l| {
        hills.push(l.chars().map(|c| c as u32 ).collect::<Vec<u32>>());

        let found_start = l.find('S').unwrap_or(l.len());
        if found_start != l.len() {
            start_pos = (line, found_start);
            hills[line][found_start] = 97;
        }
        
        let found_end = l.find('E').unwrap_or(l.len());
        if found_end != l.len() {
            end_pos = (line, found_end);
            hills[line][found_end] = 122;
        }
        
        line += 1;
    });
    let mut hill_paths: HashMap<(usize, usize), Vec<Edge>> = HashMap::new();
    for row in 0..hills.len() {
        for column in 0..hills[row].len() {
            hill_paths.insert((row, column), Vec::new());
            for neighbour in neighbours((row, column), hills[row].len(), hills.len()) {
                if hills[neighbour.0][neighbour.1] <= hills[row][column] + 1 {
                    hill_paths.get_mut(&(row, column)).unwrap().push(Edge {node: (neighbour.0, neighbour.1), cost: 1});
                }
            }
        }
    }
    let end_input = Instant::now();
    
    // First problem
    println!(
        "A: {:?}",
        shortest_path(&hill_paths, start_pos, end_pos).unwrap(),
    );
    let end_prob_a = Instant::now();

    // Second problem
    let mut start_coords: Vec<(usize, usize)> = Vec::new();
    for row in 0..hills.len() {
        for column in 0..hills[row].len() {
            if hills[row][column] == 97 {
                start_coords.push((row, column));
            }
        }
    }
    println!(
        "B: {:?}",
        start_coords.iter().map(|coords| shortest_path(&hill_paths, *coords, end_pos).unwrap_or(usize::MAX)).min().unwrap(),
    );
    let end_prob_b = Instant::now();

    let total = end_input.duration_since(start) + end_prob_a.duration_since(end_input) + end_prob_b.duration_since(end_prob_a);
    println!("Time input: {:?} ns ({:.2?}%)", end_input.duration_since(start).as_nanos(), end_input.duration_since(start).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time A: {:?} ns ({:.2?}%)", end_prob_a.duration_since(end_input).as_nanos(), end_prob_a.duration_since(end_input).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time B: {:?} ns ({:.2?}%)", end_prob_b.duration_since(end_prob_a).as_nanos(), end_prob_b.duration_since(end_prob_a).as_nanos() as f64 / total.as_nanos() as f64 * 100.0);
    println!("Time total: {:?}", total);
}