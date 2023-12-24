use std::{fs, collections::HashSet};

struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl From<&str> for Hailstone {
    fn from(s: &str) -> Self {
        let s = s.replace(" @", ", ");
        let mut values = s.split(", ").map(|s| s.trim().parse::<i64>().unwrap());
        Self {
            x: values.next().unwrap(),
            y: values.next().unwrap(),
            z: values.next().unwrap(),
            vx: values.next().unwrap(),
            vy: values.next().unwrap(),
            vz: values.next().unwrap(),
        }
    }
}

fn cross2d(lhs: &Hailstone, rhs: &Hailstone) -> Option<(f64, f64)> {
    let a = (rhs.x - lhs.x) as f64 / lhs.vx as f64;
    let b = rhs.vx as f64 / lhs.vx as f64;
    let c = (rhs.y - lhs.y) as f64 / lhs.vy as f64;
    let d = rhs.vy as f64 / lhs.vy as f64;
    let trhs = (a - c) / (d - b);
    let tlhs = a + b * trhs;
    if trhs > 0_f64 && tlhs > 0_f64 {
        Some((rhs.x as f64 + trhs * rhs.vx as f64, rhs.y as f64 + trhs * rhs.vy as f64))
    } else {
        None
    }
}

fn main() {
    let hailstones: Vec<Hailstone> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| Hailstone::from(s))
        .collect();
    let mut colliding: HashSet<usize> = HashSet::new(); 
    let mut part1 = 0;
    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            if i == j {
                continue;
            }
            if let Some((x, y)) = cross2d(&hailstones[i], &hailstones[j]) {
                if x >= 200000000000000_f64
                    && x <= 400000000000000_f64
                    && y >= 200000000000000_f64
                    && y <= 400000000000000_f64
                {
                    colliding.insert(j);
                    part1 += 1;
                }
            }
        }
    }
    println!("Part 1: {}", part1);
}
