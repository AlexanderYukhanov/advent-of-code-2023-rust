use std::fs;

fn diff(seq: &Vec<i32>) -> Vec<i32> {
    seq.iter()
        .zip(seq.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

fn extrapolate(seq: &Vec<i32>) -> i32 {
    if seq.len() == 1 {
        return seq[0];
    }
    return seq.last().unwrap() + extrapolate(&diff(seq));
}

fn extrapolate_left(seq: &Vec<i32>) -> i32 {
    if seq.len() == 1 {
        return seq[0];
    }
    return seq[0] - extrapolate_left(&diff(seq));
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let sequences: Vec<Vec<i32>> = content
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let part1: i32 = sequences.iter().map(|v| extrapolate(&v)).sum();
    println!("Part 1: {}", part1);
    let part2: i32 = sequences.iter().map(|v| extrapolate_left(&v)).sum();
    println!("Part 2: {}", part2);
}
