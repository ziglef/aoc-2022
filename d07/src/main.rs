use std::collections::HashMap;

fn main() {
    let mut dir_tree: HashMap<String, u32> = HashMap::new();
    let mut cwd: String = String::new();
    
    cwd.push('/');
    dir_tree.insert(String::from("/"), 0);

    include_str!("../input.txt").lines().for_each(|l| {
        // parse commands
        if l.starts_with('$') {
            let mut args = l.split_whitespace();
            args.next();
            if args.next().unwrap() == "cd" {
                let folder = args.next().unwrap();
                if folder == ".." {
                    cwd.drain(cwd.rfind('/').unwrap()..).for_each(drop);
                } else {
                    if folder != "/" {
                        if cwd != "/" {
                            cwd.push_str("/");
                        }
                        cwd.push_str(folder);
                    }
                }
            }
        // parse folders
        } else if l.starts_with("dir") {
            let folder_name = if cwd == "/" {
                format!("{}{}", "/", l.split_whitespace().skip(1).next().unwrap())
            } else { 
                format!("{}{}{}", cwd, "/", l.split_whitespace().skip(1).next().unwrap())
            };
            dir_tree.insert(folder_name, 0);
        // parse files
        } else {
            let mut args = l.split_whitespace();
            let (size, _) = (args.next().unwrap().parse::<u32>().unwrap(), args.next().unwrap());
            *dir_tree.get_mut(&cwd).unwrap() += size;
        }
    });


    // First problem
    let mut dir_sums: HashMap<String, u32> = HashMap::new();
    let mut dir_keys_desc_len = dir_tree.keys().cloned().collect::<Vec<String>>();
    dir_keys_desc_len.sort_by(|a, b| (a.matches("/").count() + a.len()).cmp(&(b.matches("/").count() + b.len())));
    for other_key in &dir_keys_desc_len {
        for key in dir_tree.keys() {    
            if key.starts_with(other_key) {
                dir_sums.entry(other_key.to_string()).and_modify(|v| *v += dir_tree.get(key).unwrap()).or_insert(*dir_tree.get(key).unwrap());
            }
        }
    }

    println!(
        "{}",
        dir_sums.values().filter(|&v| *v <= 100000).sum::<u32>(),
    );

    // Second problem
    let space_to_free = 30000000 - (70000000 - dir_sums.get("/").unwrap());
    
    println!(
        "{}",
        dir_sums.values().filter(|&v| *v >= space_to_free).min().unwrap(),
    );
}