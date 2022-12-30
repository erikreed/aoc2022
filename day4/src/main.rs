use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const PART1: bool = false;

fn main() {
    let reader = BufReader::new(File::open("day4/input.txt").unwrap());

    let mut sum = 0;

    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let parts: Vec<&str> = unwrapped.split(',').collect();
        println!("{parts:?}");
        let p1: Vec<&str> = parts[0].split('-').collect();
        let p2: Vec<&str> = parts[1].split('-').collect();
        let x1: i32 = p1[0].parse().unwrap();
        let y1: i32 = p1[1].parse().unwrap();
        let x2: i32 = p2[0].parse().unwrap();
        let y2: i32 = p2[1].parse().unwrap();

        if PART1 {
            if (x1 <= x2 && y1 >= y2) || (x2 <= x1 && y2 >= y1) {
                sum += 1;
                println!("full overlap");
            }
        } else {
            let amax = std::cmp::max(x1, y1);
            let amin = std::cmp::min(x1, y1);
            let bmax = std::cmp::max(x2, y2);
            let bmin = std::cmp::min(x2, y2);
            if (amin >= bmin && amin <= bmax) || (bmin >= amin && bmin <= amax) {
                println!("any overlap");
                sum += 1;
            }
        }
    }
    println!("{sum}");
}
