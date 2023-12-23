use std::{
    collections::HashSet,
    fs,
};

#[derive(Clone)]
struct Position {
    coords: Vec<usize>,
}

struct Brick {
    a: Position,
    b: Position,
    supporters: HashSet<usize>,
    supporting: HashSet<usize>,
}

impl Position {
    fn from_string(s: &str) -> Self {
        Self {
            coords: s.split(",").map(|s| s.parse::<usize>().unwrap()).collect(),
        }
    }
}

impl Brick {
    fn from_positions(a: Position, b: Position) -> Self {
        if a.coords.iter().zip(b.coords.iter()).any(|(c1, c2)| c1 < c2) {
            Self {
                supporters: HashSet::new(),
                supporting: HashSet::new(),
                a: a.clone(),
                b: b.clone(),
            }
        } else {
            Self {
                supporters: HashSet::new(),
                supporting: HashSet::new(),
                a: b.clone(),
                b: a.clone(),
            }
        }
    }

    fn from_string(s: &str) -> Self {
        if let Some((a, b)) = s.split_once("~") {
            return Self::from_positions(Position::from_string(a), Position::from_string(b));
        }
        panic!("Incorrect input")
    }

    fn occupied(&self, x: usize, y: usize, z: usize) -> bool {
        if z < self.a.coords[2] || z > self.b.coords[2] + 1 {
            return false;
        }

        if y < self.a.coords[1] || y > self.b.coords[1] {
            return false;
        }

        if x < self.a.coords[0] || x > self.b.coords[0] {
            return false;
        }

        return true;
    }

    fn drop(&mut self) {
        self.b.coords[2] -= self.a.coords[2] - 1;
        self.a.coords[2] = 1;
    }

    fn remove_supporter(&mut self, supporter: usize) -> bool {
        self.supporters.remove(&supporter);
        return self.supporters.is_empty();
    }

    fn add_supporter(&mut self, supporter: usize) {
        self.supporters.insert(supporter);
    }

    fn add_supported(&mut self, supported: usize) {
        self.supporting.insert(supported);
    }
}

fn count_fallings(brick: usize, bricks: &mut Vec<Brick>) -> usize {
    let mut fallen: Vec<usize> = vec![];
    let mut falling: Vec<usize> = vec![brick];
    while let Some(b) = falling.pop() {
        fallen.push(b);
        let supporting = bricks[b].supporting.clone();
        for s in supporting {
            if bricks[s].remove_supporter(b) {
                falling.push(s);
            }
        }
    }
    // restore the initial state
    for b in &fallen {
        let supporting = bricks[*b].supporting.clone();
        for s in supporting {
            bricks[s].add_supporter(*b);
        }
    }
    // -1 as fallen contains the brick itself
    return fallen.len() - 1;
}

fn main() {
    let mut bricks: Vec<Brick> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| Brick::from_string(s))
        .collect();
    bricks.sort_by_key(|br| br.a.coords[2]);
    bricks[0].drop();
    for bi in 1..bricks.len() {
        while bricks[bi].a.coords[2] > 1 {
            let mut can = true;
            for x in bricks[bi].a.coords[0]..=bricks[bi].b.coords[0] {
                for y in bricks[bi].a.coords[1]..=bricks[bi].b.coords[1] {
                    for j in 0..bi {
                        if bricks[j].occupied(x, y, bricks[bi].a.coords[2]) {
                            can = false;
                            bricks[j].add_supported(bi);
                            bricks[bi].add_supporter(j);
                        }
                    }
                }
            }
            if can {
                let br = &mut bricks[bi];
                br.a.coords[2] -= 1;
                br.b.coords[2] -= 1;
            } else {
                break;
            }
        }
    }
    let critical: HashSet<&usize> = bricks
        .iter()
        .filter_map(|b| {
            if b.supporters.len() == 1 {
                b.supporters.iter().next()
            } else {
                None
            }
        })
        .collect();
    let part1 = (0..bricks.len())
        .filter(|ind| !critical.contains(ind))
        .count();
    println!("Part1: {}", part1);
    let part2: usize = (0..bricks.len())
        .map(|b| count_fallings(b, &mut bricks))
        .sum();
    println!("Part2: {}", part2);
}
