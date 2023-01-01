use std::{
    cmp::max,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    max((a.0 - b.0).abs(), (a.1 - b.1).abs())
}

struct KnotNode {
    head_position: (i32, i32),
    tail_position: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl KnotNode {
    fn update_head(&mut self, new_position: (i32, i32)) {
        if distance(new_position, self.tail_position) > 1 {
            self.tail_position = self.head_position;
            self.visited.insert(self.tail_position);
        }
        self.head_position = new_position;
    }
    fn new() -> KnotNode {
        let mut new = KnotNode {
            head_position: (0, 0),
            tail_position: (0, 0),
            visited: HashSet::new(),
        };
        new.visited.insert(new.tail_position);
        return new;
    }
}

fn update_knots(knots: &mut Vec<KnotNode>, head_position: (i32, i32)) {
    knots[0].update_head(head_position);
    for i in 1..knots.len() {
        let t = knots[i - 1].tail_position;
        knots[i].update_head(t);
    }
}

const NUM_KNOTS: i32 = 10;

fn main() {
    let reader = BufReader::new(File::open("day9/input.txt").unwrap());

    let mut knots: Vec<KnotNode> = Vec::new();
    for _ in 0..NUM_KNOTS {
        knots.push(KnotNode::new());
    }

    let mut head_position = (0, 0);

    for line in reader.lines() {
        let parts: Vec<&str> = line.as_ref().unwrap().split(' ').collect();
        let dir = parts[0].chars().next().unwrap();
        let magnitude: i32 = parts[1].parse().unwrap();

        let (dx, dy) = match dir {
            'U' => (0, magnitude),
            'D' => (0, -magnitude),
            'L' => (-magnitude, 0),
            'R' => (magnitude, 0),
            _ => panic!("malformed"),
        };

        for _ in 0..dx.abs() {
            head_position.0 += if dx.is_positive() { 1 } else { -1 };
            update_knots(&mut knots, head_position);
        }

        for _ in 0..dy.abs() {
            head_position.1 += if dy.is_positive() { 1 } else { -1 };
            update_knots(&mut knots, head_position);
        }
    }
    let visited_count = knots.last().unwrap().visited.len();
    for knot in &knots {
        dbg!(knot.tail_position);
    }
    println!("visited_count: {visited_count}");
}
