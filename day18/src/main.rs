use std::collections::BTreeMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum LineState {
    OPEN,
    CLOSE,
    CONTINUE,
}

struct Instruction {
    dir: char,
    dist: usize,
}

fn delta(c: char) -> (i64, i64) {
    match c {
        'U' => (0, -1),
        'D' => (0, 1),
        'L' => (-1, 0),
        _ => (1, 0),
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    state: LineState,
}

impl Point {
    fn open(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
            state: LineState::OPEN,
        }
    }

    fn close(x: i64, y: i64) -> Self {
        Self {
            x,
            y,
            state: LineState::CLOSE,
        }
    }
}

struct Solver<'a> {
    instructions: &'a Vec<Instruction>,
    points: Vec<Point>,
    pnt: usize,
    line: BTreeMap<i64, Point>,
}

impl<'a> Solver<'a> {
    fn new(insts: &'a Vec<Instruction>) -> Self {
        Self {
            instructions: insts,
            points: vec![],
            pnt: 0,
            line: BTreeMap::new(),
        }
    }

    fn measure(line: &BTreeMap<i64, Point>) -> i64 {
        let mut r = 0;
        let mut inside = false;
        let mut prev = &Point {
            x: 0,
            y: 0,
            state: LineState::CONTINUE,
        };
        let mut open = false;
        for p in line.values() {
            match (prev.state, open, inside, p.state) {
                (_, _, true, LineState::CONTINUE) => {
                    r += p.x - prev.x;
                    inside = false;
                }
                (_, _, false, LineState::CONTINUE) => {
                    r += 1;
                    inside = true;
                    prev = p;
                }
                (_, false, true, LineState::CLOSE) => {
                    r += p.x - prev.x;
                    prev = p;
                    open = true;
                }
                (_, false, true, LineState::OPEN) => {
                    r += p.x - prev.x;
                    prev = p;
                    open = true;
                }
                (_, false, false, LineState::CLOSE) => {
                    r += 1;
                    prev = p;
                    open = true;
                }
                (_, false, false, LineState::OPEN) => {
                    r += 1;
                    prev = p;
                    open = true;
                }
                (LineState::OPEN, true, _, LineState::OPEN) => {
                    r += p.x - prev.x;
                    prev = p;
                    open = false;
                }
                (LineState::CLOSE, true, _, LineState::CLOSE) => {
                    r += p.x - prev.x;
                    prev = p;
                    open = false;
                }
                (LineState::CLOSE, true, _, LineState::OPEN) => {
                    r += p.x - prev.x;
                    prev = p;
                    open = false;
                    inside = !inside;
                }
                (LineState::OPEN, true, _, LineState::CLOSE) => {
                    r += p.x - prev.x;
                    prev = p;
                    open = false;
                    inside = !inside;
                }
                _ => {}
            }
        }
        r
    }

    fn solve(&mut self) -> i64 {
        let mut result = 0;
        self.analyze_path();
 
        let mut last = self.calculate_next_line().unwrap();
        result += Self::measure(&self.line);
        
        self.finalyze_line();
        let mut per_line = Self::measure(&self.line);
  
        while let Some(y) = self.calculate_next_line() {
            result += (y - last - 1) * per_line;
            result += Self::measure(&self.line);
            
            self.finalyze_line();
            per_line = Self::measure(&self.line);

            last = y;
        }
        result
    }

    fn analyze_path(&mut self) {
        let mut x = 0;
        let mut y = 0;
        self.instructions.iter().for_each(|inst| {
            let d = delta(inst.dir);
            let nx = x + d.0 as i64 * inst.dist as i64;
            let ny = y + d.1 as i64 * inst.dist as i64;
            if inst.dir == 'U' {
                self.points.push(Point::open(x, ny));
                self.points.push(Point::close(x, y));
            } else if inst.dir == 'D' {
                self.points.push(Point::open(x, y));
                self.points.push(Point::close(x, ny));
            };
            x = nx;
            y = ny;
        });
        self.points.sort_by_key(|pt| pt.y);
    }

    fn calculate_next_line(&mut self) -> Option<i64> {
        if self.pnt == self.points.len() {
            return None;
        }
        let y = self.points[self.pnt].y;
        while self.pnt < self.points.len() && self.points[self.pnt].y == y {
            let pt = self.points[self.pnt];
            self.pnt += 1;
            self.line.insert(pt.x, pt.clone());
        }
        Some(y)
    }

    fn finalyze_line(&mut self) {
        let preserved: Vec<Point> = self
            .line
            .values()
            .filter(|v| v.state != LineState::CLOSE)
            .cloned()
            .collect();
        self.line = BTreeMap::new();
        for p in preserved {
            self.line.insert(
                p.x,
                Point {
                    x: p.x,
                    y: p.y,
                    state: LineState::CONTINUE,
                },
            );
        }
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let instructions: Vec<Instruction> = content
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|l| {
            let mut parts = l.split(" ");
            Instruction {
                dir: parts.next().unwrap().chars().next().unwrap(),
                dist: parts.next().unwrap().parse::<usize>().unwrap(),
            }
        })
        .collect();
    println!("Part 1: {}", Solver::new(&instructions).solve());

    let instructions2: Vec<Instruction> = content
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|l| {
            if let Some((_, s)) = l.split_once("#") {
                let hex: String = s.chars().take(5).collect();
                let dist = usize::from_str_radix(hex.as_str(), 16).unwrap();
                let dir = s.chars().skip(5).next().unwrap();
                let dir = match dir {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    _ => 'U',
                };
                Instruction { dir, dist }
            } else {
                panic!("Unexpected input");
            }
        })
        .collect();
    println!("Part 2: {}", Solver::new(&instructions2).solve());
}
