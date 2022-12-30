use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main_p1() {
    let reader = BufReader::new(File::open("day3/input.txt").unwrap());

    let mut sum = 0;
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        let chars = unwrapped.as_bytes();
        let p1 = &chars[..chars.len() / 2];
        let p2 = &chars[chars.len() / 2..];

        let mut found = [false; 64];
        for c in p1 {
            let idx = *c as usize - 'A' as usize;
            found[idx] = true;
        }
        for c in p2 {
            let idx = *c as usize - 'A' as usize;
            if found[idx] {
                sum += if *c >= 'a' as u8 {
                    *c as i32 - 'a' as i32 + 1
                } else {
                    *c as i32 - 'A' as i32 + 27
                };
                break;
            }
        }
    }
    println!("{sum}");
}

fn main() {
    let reader = BufReader::new(File::open("day3/input.txt").unwrap());

    let mut sum = 0;
    let mut line_idx = 0;
    let mut found = [0; 256];

    for line in reader.lines() {
        line_idx += 1;

        let unwrapped = line.unwrap();
        let chars = unwrapped.as_bytes();

        let mut found_inner = [false; 256];
        for c in chars {
            found_inner[*c as usize] = true;
        }
        for (i, c) in found_inner.iter().enumerate() {
            if *c {
                found[i] += 1;
            }
        }

        println!("{unwrapped}");

        if line_idx == 3 {
            for (i, c) in found.iter().enumerate() {
                if *c == 3 {
                    let ch = char::from_u32(i as u32).unwrap();
                    sum += if ch >= 'a' {
                        ch as i32 - 'a' as i32 + 1
                    } else {
                        ch as i32 - 'A' as i32 + 27
                    };
                    println!("{ch}: {sum}");
                    break;
                }
            }
            found = [0; 256];
            line_idx = 0;
        }
    }
    println!("{sum}");
}
