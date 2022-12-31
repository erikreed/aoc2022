use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("day7/input.txt").unwrap());

    let mut lines = reader.lines();
    // skip initial 'cd /'
    lines.next();

    let mut path_components: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();
    let mut total_size = 0;

    for line in lines {
        let line = line.unwrap();

        if line.starts_with("$ cd ") {
            let path = line.split("$ cd ").last().unwrap();
            if path == ".." {
                path_components.pop().expect("malformed");
            } else {
                path_components.push(path.to_string());
            }
        } else if line == "$ ls" {
            // no use for tracking ls command it seems
        } else {
            let parts: Vec<&str> = line.split(" ").collect();
            assert!(parts.len() == 2);
            let size: i32 = match parts[0].parse() {
                Ok(val) => val,
                // skip the 'dir ' entries
                Err(_) => continue,
            };
            total_size += size;
            let mut absolute_path = String::new();
            for component in &path_components {
                absolute_path += "/";
                absolute_path += component;
                let existing_size = dir_sizes.get(&absolute_path).unwrap_or(&0);
                dir_sizes.insert(absolute_path.to_owned(), existing_size + size);
            }
        }
    }

    let sum_below_100k: i32 = dir_sizes.values().filter(|s| **s <= 100_000).sum();
    println!("sum_below_100k: {sum_below_100k}");

    let used_space = total_size;
    let free_space = 70_000_000 - used_space;
    let min_to_free = 30_000_000 - free_space;

    let min_dir_to_free = dir_sizes.values().filter(|s| **s >= min_to_free).min().unwrap();
    println!("min_dir_to_free: {min_dir_to_free}");
}
