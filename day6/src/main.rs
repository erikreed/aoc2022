use std::collections::HashSet;

const MATCH_SIZE: usize = 14;

fn main() {
    let contents = std::fs::read_to_string("day6/input.txt").unwrap();
    for mut i in 0..contents.len() - MATCH_SIZE {
        let chunk = &contents[i..i+MATCH_SIZE];
        let unique: HashSet<char> = chunk.chars().collect();
        if unique.len() == MATCH_SIZE {
            i += MATCH_SIZE;
            println!("{i}");
            break;
        }
    }
}
