use std::{cmp, collections::HashSet, fs};

fn dfs(
    filter: fn(char, i32, i32) -> bool,
    x: i32,
    y: i32,
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<i32>,
) -> Option<usize> {
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
            let allowed = filter(map[ny as usize][nx as usize], dx, dy);
            if allowed && visited.insert(ny * 255 + nx) {
                if let Some(r) = dfs(filter, nx, ny, map, visited) {
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

fn filter_scopes(ch: char, dx: i32, dy: i32) -> bool {
    match (ch, dx, dy) {
        ('#', _, _) => false,
        ('.', _, _) => true,
        ('^', _, 1) => true,
        ('v', _, -1) => true,
        ('<', 1, _) => true,
        ('>', -1, _) => true,
        _ => false,
    }
}

fn filter_walls(ch: char, _: i32, _: i32) -> bool {
    ch != '#'
}

fn main() {
    let field: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    let mut visited = &mut HashSet::new();
    let x = field[0].len() as i32 - 2;
    let y = field.len() as i32 - 1;
    visited.insert(y * 255 + x);
    let part1 = dfs(filter_scopes, x, y, &field, &mut visited);
    println!("Part 1: {:?}", part1);
    let part2 = dfs(filter_walls, x, y, &field, &mut visited);
    println!("Part 2: {:?}", part2);
}
