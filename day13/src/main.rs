use std::fs;

struct OriginalMap<'a> {
    items: &'a Vec<Vec<char>>,
}

struct RotatedMap<'a> {
    items: &'a Vec<Vec<char>>,
}

trait GameMap {
    fn rows(&self) -> usize;
    fn columns(&self) -> usize;
    fn get(&self, r: usize, c: usize) -> char;
}

impl GameMap for OriginalMap<'_> {
    fn rows(&self) -> usize {
        self.items.len()
    }

    fn columns(&self) -> usize {
        self.items[0].len()
    }

    fn get(&self, r: usize, c: usize) -> char {
        self.items[r][c]
    }
}

impl GameMap for RotatedMap<'_> {
    fn rows(&self) -> usize {
        self.items[0].len()
    }

    fn columns(&self) -> usize {
        self.items.len()
    }

    fn get(&self, r: usize, c: usize) -> char {
        self.items[c][r]
    }
}

impl<'a> OriginalMap<'a> {
    fn new(items: &'a Vec<Vec<char>>) -> Self {
        Self { items: items }
    }
}

impl<'a> RotatedMap<'a> {
    fn new(items: &'a Vec<Vec<char>>) -> Self {
        Self { items: items }
    }
}

fn count_smudges_in_row_mirrored_at(map: &dyn GameMap, r: usize, c: usize) -> usize {
    let mut lhs = c - 1;
    let mut rhs = c;
    let mut result = 0;
    loop {
        if map.get(r, lhs) != map.get(r, rhs) {
            result += 1;
        }
        if lhs == 0 || rhs == map.columns() - 1 {
            break;
        }
        lhs -= 1;
        rhs += 1;
    }
    return result;
}

fn count_smudges_for_map_mirrored_at(map: &dyn GameMap, c: usize) -> usize {
    (0..map.rows())
        .map(|r| count_smudges_in_row_mirrored_at(map, r, c))
        .sum()
}

fn count_mirrors(map: &dyn GameMap, smudges: usize) -> usize {
    (1..map.columns())
        .filter(|c| count_smudges_for_map_mirrored_at(map, *c) == smudges)
        .sum()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content = content
        .split("\n")
        .map(|s| s.trim())
        .chain(std::iter::once(""));
    let mut map: Vec<Vec<char>> = vec![];
    let mut part1 = 0;
    let mut part2 = 0;
    for ln in content {
        if ln.is_empty() {
            part1 += count_mirrors(&OriginalMap::new(&map), 0);
            part1 += count_mirrors(&RotatedMap::new(&map), 0) * 100;
            part2 += count_mirrors(&OriginalMap::new(&map), 1);
            part2 += count_mirrors(&RotatedMap::new(&map), 1) * 100;

            map = vec![];
        } else {
            map.push(ln.chars().collect::<Vec<char>>());
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
