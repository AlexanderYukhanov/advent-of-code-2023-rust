use std::{fs, collections::HashSet, cmp};

fn dfs(x: i32, y: i32, map: &Vec<Vec<char>>, visited: &mut HashSet<i32>) -> Option<usize> {
    if x == 1 && y == 0 {
        return Some(0);
    }
    let mut found = false;
    let mut result = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx != 0 && dy != 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx == map[0].len() as i32 || ny < 0 || ny == map.len() as i32 {
                continue;
            }
            let ch = map[ny as usize][nx as usize];
            let allowed = match (ch, dx, dy) {
                ('#', _, _) => false,
                ('.', _, _) => true,
                ('^', _, 1) => true,
                ('v', _, -1) => true,
                ('<', 1, _) => true,
                ('>', -1, _) => true,
                _ => false,
            };
            if allowed && visited.insert(ny * 255 + nx){
                if let Some(r) = dfs(nx, ny, map, visited) {
                    found = true;
                    result = cmp::max(result, r);
                }
                visited.remove(&(ny * 255 + nx));
            }
        }
    }
    if found {
        Some(result + 1)
    } else {
        None
    }
}

fn main() {
    let field: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    let mut visited =  &mut HashSet::new();
    let x = field[0].len() as i32 - 2;
    let y = field.len() as i32 - 1;
    visited.insert(y * 255 + x);
    println!("{}:{}", x, y);
    let part1 = dfs(x, y, &field, &mut visited);
    println!("Part 1: {:?}", part1);
}
