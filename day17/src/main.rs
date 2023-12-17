use priority_queue::PriorityQueue;
use std::{collections::HashSet, fs};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
    INITIAL,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    r: usize,
    c: usize,
    dir: Direction,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Direction::UP,
            1 => Direction::RIGHT,
            2 => Direction::DOWN,
            3 => Direction::LEFT,
            _ => Direction::INITIAL,
        }
    }
}

impl Direction {
    fn advance(self, p: &mut Position) {
        match self {
            Direction::UP => p.r = p.r.wrapping_sub(1),
            Direction::RIGHT => p.c += 1,
            Direction::DOWN => p.r += 1,
            Direction::LEFT => p.c = p.c.wrapping_sub(1),
            Direction::INITIAL => {}
        };
    }
    fn backward(self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::RIGHT => Direction::LEFT,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::INITIAL => Direction::INITIAL,
        }
    }
}

impl Position {
    fn new(r: usize, c: usize, dir: Direction) -> Self {
        Self { r, c, dir }
    }

    fn rotate(&self) -> Vec<Position> {
        (0_u8..4)
            .map(|v| Direction::from(v))
            .filter(|dir| *dir != self.dir)
            .filter(|dir| dir.backward() != self.dir)
            .map(|dir| Position::new(self.r, self.c, dir))
            .collect()
    }

    fn advance(&self, dist: i32, delta: usize, field: &Vec<Vec<i32>>) -> Option<(Position, i32)> {
        let mut t = self.clone();
        let mut d = dist;
        for _ in 0..delta {
            self.dir.advance(&mut t);
            if t.r >= field.len() || t.c >= field[0].len() {
                return None;
            }
            d += field[t.r][t.c];
        }
        Some((t, d))
    }
}

struct PathFinder<'a> {
    ranges: Vec<usize>,
    added: HashSet<Position>,
    pending: PriorityQueue<Position, i32>,
    field: &'a Vec<Vec<i32>>,
    height: usize,
    widht: usize,
}

impl<'a> PathFinder<'a> {
    fn new(field: &'a Vec<Vec<i32>>, ranges: Vec<usize>) -> Self {
        let mut me = Self {
            ranges: ranges,
            added: HashSet::new(),
            pending: PriorityQueue::new(),
            field: field,
            height: field.len(),
            widht: field[0].len(),
        };
        me.pending.push(Position::new(0, 0, Direction::INITIAL), 0);
        me
    }

    fn discover(&mut self, pos: Position, path: i32) {
        pos.rotate()
            .iter()
            .map(|p| {
                self.ranges
                    .iter()
                    .map(|delta| p.advance(path, *delta, self.field))
                    .filter(|v| v.is_some())
                    .map(|v| v.unwrap())
            })
            .flatten()
            .filter(|(p, _)| p.r < self.height && p.c < self.widht)
            .filter(|(p, _)| !self.added.contains(p))
            .for_each(|(p, dist)| {
                if let Some(old) = self.pending.get_priority(&p) {
                    if *old < -dist {
                        self.pending.change_priority(&p, -dist);
                    }
                } else {
                    self.pending.push(p, -dist);
                }
            });
    }

    fn find(&mut self) -> Option<i32> {
        let mut best: Option<i32> = None;
        while let Some((pos, dist)) = self.pending.pop() {
            if pos.r == self.height - 1 && pos.c == self.widht - 1 {
                if let Some(old) = best {
                    if old > -dist {
                        best = Some(-dist);
                    }
                } else {
                    best = Some(-dist);
                }
            }
            self.added.insert(pos);
            self.discover(pos, -dist);
        }
        best
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let field: Vec<Vec<i32>> = content
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    println!(
        "Part 1: {:?}",
        PathFinder::new(&field, vec![1, 2, 3]).find()
    );
    println!(
        "Part 2: {:?}",
        PathFinder::new(&field, (4..=10).collect()).find()
    );
}
