fn main() {
    let mut elf_pairs: Vec<((u32, u32), (u32, u32))> = Vec::new();
    
    include_str!("../input.txt")
        .lines()
        .for_each(|l| {
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
    
    // First problem
    println!(
        "{:?}",
        elf_pairs.iter().filter(|pair| {
            (pair.1.0 >= pair.0.0 && pair.1.1 <= pair.0.1) ||
            (pair.0.0 >= pair.1.0 && pair.0.1 <= pair.1.1)
        }).count(),
    );

    // Second problem

    println!(
        "{}",
        elf_pairs.iter().filter(|pair| {
            (pair.0.0 >= pair.1.0 && pair.0.0 <= pair.1.1) ||
            (pair.0.1 >= pair.1.0 && pair.0.1 <= pair.1.1) ||
            (pair.1.0 >= pair.0.0 && pair.1.0 <= pair.0.1) ||
            (pair.1.1 >= pair.0.0 && pair.1.1 <= pair.0.1)
        }).count(),
    );
}