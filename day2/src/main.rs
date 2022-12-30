use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn mainP1() {
    let reader = BufReader::new(File::open("day2/input.txt").unwrap());

    let mut sum = 0;
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let chars = unwrapped.as_bytes();
        let (a, b) = (chars[0] as char, chars[2] as char);
        println!("{a} {b}");

        // ABC = rock/paper/scissors
        // XYZ = rock/paper/scissors
        let mut score = match (a, b) {
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            _ => 6
        };
        score += (b as i32 - 'X' as i32) + 1;
        sum += score;
        println!("{score}");
    }
    println!("{sum}");
}

fn main() {
    let reader = BufReader::new(File::open("day2/input.txt").unwrap());

    let mut sum = 0;
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let chars = unwrapped.as_bytes();
        let (a, mut b) = (chars[0] as char, chars[2] as char);
        println!("{a} {b}");

        // ABC = rock/paper/scissors
        // XYZ = rock/paper/scissors
        // XYZ = lose/draw/win
        match b {
            'Z' => {
                b = match a {
                    'A' => 'Y',
                    'B' => 'Z',
                    'C' => 'X',
                    _ => panic!()
                };
            },
            'Y' => {
                b = char::from_u32((a as u32 - 'A' as u32) + 'X' as u32).unwrap();
            },
            'X' => {
                b = match a {
                    'A' => 'Z',
                    'B' => 'X',
                    'C' => 'Y',
                    _ => panic!()
                };
            },
            _ => {
                panic!()
            }
        }
        println!("{a} {b}");

        // ABC = rock/paper/scissors
        let mut score = match (a, b) {
            ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            _ => 6
        };
        score += (b as i32 - 'X' as i32) + 1;
        sum += score;
        println!("{score}");
    }
    println!("{sum}");
}

