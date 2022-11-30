fn main() {
    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;

    include_str!("../input.txt")
        .lines()
        .for_each(|i| {
            let mut split = i.split_whitespace();
            let (dir, amt): (&str, i32) = (split.next().unwrap(), split.next().unwrap().parse::<i32>().unwrap());

            match dir {
                "forward" => horizontal += amt,
                "down" => vertical += amt,
                "up" => vertical -= amt,
                _ => println!("An error occurred"),
            }
        });

    println!(
        "{}",
        horizontal * vertical
    );
}