use std::collections::HashMap;
use std::fs;

fn step<'a>(start: &str, dir: char, paths: &'a HashMap<String, (String, String)>) -> &'a str {
    let dsts = paths.get(start).unwrap();
    if dir == 'L' {
        return &dsts.0;
    }
    return &dsts.1;
}

fn main() {
    let mut paths: HashMap<String, (String, String)> = HashMap::new();
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let mut dirs = lines[0].chars().cycle();
    lines.iter().skip(1).for_each(|l| {
        let clean: String = l
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ')
            .collect();
        let mut parts = clean.split(" ").filter(|s| !s.is_empty());
        let src = parts.next().unwrap().to_owned();
        paths.insert(
            src,
            (
                parts.next().unwrap().to_owned(),
                parts.next().unwrap().to_owned(),
            ),
        );
    });
    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        current = step(current, dirs.next().unwrap(), &paths);
        steps += 1;
    }
    println!("Part 1: {}", steps);
    let mut ghosts: Vec<&str> = paths
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|v| v.as_str())
        .collect();
    steps = 0_usize;
    dirs = lines[0].chars().cycle();
    let adirs: Vec<char> = lines[0].chars().collect();
    let mut di = 0_usize;
    let original = ghosts.clone();
    while !ghosts.iter().all(|s| s.ends_with("Z")) {
        let dir = dirs.next().unwrap();
        ghosts
            .iter_mut()
            .for_each(|pos| *pos = step(pos, dir, &paths));

        steps += 1;
        if di == 0 {
            for (i, v) in ghosts.iter().enumerate() {
                if v.contains(original[i]) {
                    println!("{}: {}", i, steps)
                }
            }
        }
        di += 1;
        if di == adirs.len() {
            di = 0;
        }
    }
    println!("Part 2: {}", steps);
}
