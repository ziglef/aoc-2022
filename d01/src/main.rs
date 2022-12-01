fn main() {
    let mut elfs: Vec<u32> = Vec::new();
    let mut elf_sum: u32 = 0;

    include_str!("../input.txt")
        .lines()
        .for_each(|i| {
            let amt: u32 = i.parse::<u32>().unwrap_or(0);

            if amt != 0 {
                elf_sum += amt;
            } else {
                elfs.push(elf_sum);
                elf_sum = 0;
            } 
        });

    elfs.sort();
    println!(
        "top elf: {}\nsum: {}",
        elfs.last().unwrap(),
        elfs.iter().rev().take(3).sum::<u32>()
    );
}