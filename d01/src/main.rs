fn main() {
    let mut maxs: [i32; 3] = [0,0,0];
    let mut elf_sum: i32 = 0;

    include_str!("../input.txt")
        .lines()
        .for_each(|i| {
            let amt: i32 = i.parse::<i32>().unwrap_or(0);

            if amt != 0 {
                elf_sum += amt;
            } else {
                println!("{}", elf_sum);
                for (index, value) in maxs.clone().iter().enumerate() {
                    if elf_sum > *value {
                        if index < maxs.len() - 1 {
                            if *value != 0 {
                                for (index, value) in maxs.clone().iter().enumerate().rev() {
                                    if index != maxs.len() - 1 {
                                        maxs[index+1] = *value;
                                    }
                                }
                            }
                        }
                        maxs[index] = elf_sum;
                        break;
                    }
                }
                println!("{:?}", maxs);
                elf_sum = 0;
            }

            
        });

    println!(
        "{:?}\nsum: {}",
        maxs, maxs.iter().sum::<i32>()
    );
}