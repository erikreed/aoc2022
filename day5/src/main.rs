use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: bool = false;

fn main() {
    let reader = BufReader::new(File::open("day5/input.txt").unwrap());

    let mut crates: [Vec<char>; 9] = Default::default();

    let mut lines = reader.lines();
    for _l in 1..9 {
        let unwrapped = lines.next().unwrap().unwrap();
        let chars: Vec<char> = unwrapped.chars().collect();

        for i in 0..9 {
            let c = chars[i * 4 + 1];
            if c != ' ' {
                crates[i].push(c);
            }
        }
    }
    for c in &mut crates {
        c.reverse();
    }
    lines.next();
    lines.next();

    for line in lines {
        let unwrapped = line.unwrap();
        let parts: Vec<&str> = unwrapped.split(' ').collect();
        println!("{unwrapped}");

        let move_count: usize = parts[1].parse().unwrap();
        let move_from: usize = parts[3].parse().unwrap();
        let move_to: usize = parts[5].parse().unwrap();
        for v in &crates {
            println!("{v:?}");
        }
        println!();
        if PART1 {
            for _ in 0..move_count {
                let c = crates[move_from - 1].pop().unwrap();
                crates[move_to - 1].push(c);
            }
        } else {
            let to_move = &crates[move_from - 1].split_off(&crates[move_from - 1].len() - move_count);
            crates[move_to - 1].extend_from_slice(to_move);
        }
    }
    for c in &crates {
        let last = c.last().unwrap_or(&' ');
        print!("{last}");
    }
    println!();
}
