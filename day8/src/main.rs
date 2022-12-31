use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("day8/input.txt").unwrap());

    let mut grid: Vec<Vec<i8>> = Vec::new();
    let mut visible: Vec<Vec<bool>> = Vec::new();

    for line in reader.lines() {
        let row: Vec<i8> = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i8)
            .collect();
        visible.push(vec![false; row.len()]);
        grid.last().map(|r| assert!(r.len() == row.len()));
        grid.push(row);
    }

    // left
    for i in 0..grid.len() {
        let mut current_max: i8 = -1;

        for (j, e) in grid[i].iter().enumerate() {
            if current_max < *e {
                current_max = *e;
                visible[i][j] = true;
            }
        }
    }

    // right
    for i in 0..grid.len() {
        let mut current_max: i8 = -1;

        for (j, e) in grid[i].iter().enumerate().rev() {
            if current_max < *e {
                current_max = *e;
                visible[i][j] = true;
            }
        }
    }

    // top
    for j in 0..grid[0].len() {
        let mut current_max: i8 = -1;

        for i in 0..grid.len() {
            let e = grid[i][j];
            if current_max < e {
                current_max = e;
                visible[i][j] = true;
            }
        }
    }

    // bottom
    for j in 0..grid[0].len() {
        let mut current_max: i8 = -1;

        for i in (0..grid.len()).rev() {
            let e = grid[i][j];
            if current_max < e {
                current_max = e;
                visible[i][j] = true;
            }
        }
    }
    let count: usize = visible
        .iter()
        .map(|v| v.iter().filter(|e| **e).count())
        .sum();
    println!("visible tree count: {count}");

    let mut highest_scenic_score = -1;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            let mut scenic_score_total = 1;
            let mut scenic_score = 0;
            let val = grid[i][j];
            // up
            for i in (0..i).rev() {
                let next = grid[i][j];
                scenic_score += 1;
                if next >= val {
                    break;
                }
            }
            scenic_score_total *= scenic_score;
            scenic_score = 0;

            // down
            for i in i+1..grid.len() {
                let next = grid[i][j];
                scenic_score += 1;
                if next >= val {
                    break;
                }
            }
            scenic_score_total *= scenic_score;
            scenic_score = 0;

            // left
            for j in (0..j).rev() {
                let next = grid[i][j];
                scenic_score += 1;
                if next >= val {
                    break;
                }
            }
            scenic_score_total *= scenic_score;
            scenic_score = 0;

            // right
            for j in (j+1..grid[i].len()) {
                let next = grid[i][j];
                scenic_score += 1;
                if next >= val {
                    break;
                }
            }
            scenic_score_total *= scenic_score;

            highest_scenic_score = max(scenic_score_total, highest_scenic_score);
        }
    }
    println!("highest_scenic_score: {highest_scenic_score}");
    // 230400 too low
    // 292383 too high
}
