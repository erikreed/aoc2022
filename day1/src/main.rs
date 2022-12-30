use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut counts: Vec<i32> = Vec::new();

    let mut count = 0;
    for line in lines {
        if line.len() == 0 {
            counts.push(count);
            count = 0;
        } else {
            count += line.parse::<i32>().expect("malformed");
        }
    }
    println!("{counts:?}");
    let max = counts.iter().max().unwrap();
    println!("{max:?}");
    counts.sort_unstable();
    let top3: i32 = counts[counts.len()-3..].iter().sum();
    println!("{top3:?}");
}
