use anyhow::{Context, Result};
use std::{char, collections::HashSet, fmt::Debug, fs};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn advance(self, p: Position, maxr: usize, maxc: usize) -> Option<Position> {
        match self {
            Direction::UP => {
                if p.r != 0 {
                    Some(Position { r: p.r - 1, c: p.c })
                } else {
                    None
                }
            }
            Direction::DOWN => {
                if p.r != maxr {
                    Some(Position { r: p.r + 1, c: p.c })
                } else {
                    None
                }
            }
            Direction::LEFT => {
                if p.c != 0 {
                    Some(Position { r: p.r, c: p.c - 1 })
                } else {
                    None
                }
            }
            Direction::RIGHT => {
                if p.c != maxc {
                    Some(Position { r: p.r, c: p.c + 1 })
                } else {
                    None
                }
            }
        }
    }

    fn reflect(self, c: char) -> Vec<Direction> {
        match (self, c) {
            (Direction::UP, '|') => vec![self],
            (Direction::DOWN, '|') => vec![self],
            (Direction::LEFT, '-') => vec![self],
            (Direction::RIGHT, '-') => vec![self],

            (Direction::UP, '-') => vec![Direction::LEFT, Direction::RIGHT],
            (Direction::DOWN, '-') => vec![Direction::LEFT, Direction::RIGHT],
            (Direction::LEFT, '|') => vec![Direction::UP, Direction::DOWN],
            (Direction::RIGHT, '|') => vec![Direction::UP, Direction::DOWN],

            (Direction::UP, '/') => vec![Direction::RIGHT],
            (Direction::DOWN, '/') => vec![Direction::LEFT],
            (Direction::LEFT, '/') => vec![Direction::DOWN],
            (Direction::RIGHT, '/') => vec![Direction::UP],

            (Direction::UP, '\\') => vec![Direction::LEFT],
            (Direction::DOWN, '\\') => vec![Direction::RIGHT],
            (Direction::LEFT, '\\') => vec![Direction::UP],
            (Direction::RIGHT, '\\') => vec![Direction::DOWN],

            _ => vec![self],
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::UP => '^',
            Direction::DOWN => 'v',
            Direction::LEFT => '<',
            Direction::RIGHT => '>',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    r: usize,
    c: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    pos: Position,
    dir: Direction,
}

impl Beam {
    fn new(r: usize, c: usize, d: Direction) -> Self {
        Self {
            pos: Position { r, c },
            dir: d,
        }
    }
}

struct BeamTracer<'a> {
    pending: Vec<Beam>,
    processed: HashSet<Beam>,
    field: &'a Vec<Vec<char>>,
}

impl<'a> BeamTracer<'a> {
    fn new(field: &'a Vec<Vec<char>>, enter: Beam) -> Self {
        Self {
            pending: vec![enter.clone()],
            processed: HashSet::new(),
            field: field,
        }
    }

    fn trace_beam(&mut self, beam: Beam) {
        if !self.processed.insert(beam) {
            return;
        }
        let reflected = beam.dir.reflect(self.field[beam.pos.r][beam.pos.c]);
        for dir in reflected {
            if let Some(pos) = dir.advance(beam.pos, self.field.len() - 1, self.field[0].len() - 1)
            {
                let nb = Beam { pos, dir };
                if !self.processed.contains(&nb) {
                    self.pending.push(nb);
                }
            }
        }
    }

    fn trace(&mut self) {
        while let Some(beam) = self.pending.pop() {
            self.trace_beam(beam);
        }
    }

    fn count_energized(&mut self) -> usize {
        self.trace();
        HashSet::<&Position>::from_iter(self.processed.iter().map(|p| &p.pos)).len()
    }
}

fn main() -> Result<()> {
    let content = fs::read_to_string("input.txt").context("reading input")?;
    let field: Vec<Vec<char>> = content
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    println!(
        "Part 1: {}",
        BeamTracer::new(&field, Beam::new(0, 0, Direction::RIGHT)).count_energized()
    );
    let part2 = (0..field.len())
        .map(|v| {
            [
                Beam::new(0, v, Direction::DOWN),
                Beam::new(field.len() - 1, v, Direction::UP),
                Beam::new(v, 0, Direction::RIGHT),
                Beam::new(v, field[0].len() - 1, Direction::LEFT),
            ]
        })
        .flatten()
        .map(|enter| BeamTracer::new(&field, enter).count_energized())
        .max()
        .context("unreachable - there is a result for each enter")?;
    println!("Part 2: {}", part2);
    Ok(())
}
