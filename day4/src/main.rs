use std::{cmp, collections::HashSet, fs};

fn count_wins(card: &str) -> usize {
    let content = card.split(":").nth(1).unwrap().trim();
    let winning: HashSet<usize> = content
        .split("|")
        .nth(0)
        .unwrap()
        .split(" ")
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.parse().unwrap())
        .collect();
    return content
        .split("|")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.parse().unwrap())
        .filter(|v| winning.contains(v))
        .count();
}

fn part1(content: &str) -> usize {
    let mut total = 0;
    for line in content.split("\n") {
        let won = count_wins(line);
        if won > 0 {
            total += 1 << (won - 1);
        }
    }
    return total;
}

fn part2(content: &str) -> usize {
    let cards: Vec<&str> = content.split("\n").collect();
    let mut obtained = vec![1; cards.len()];
    for i in 0..obtained.len() {
        let won = count_wins(cards[i]);
        for u in i + 1..=cmp::min(i + won, obtained.len()) {
            obtained[u] += obtained[i];
        }
    }
    obtained.iter().sum()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}
