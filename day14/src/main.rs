use std::{collections::HashMap, fs};

struct Platform<'a> {
    rotation: i32,
    content: &'a mut Vec<Vec<char>>,
    states: HashMap<String, usize>,
    period: Option<(usize, usize)>,
}

impl<'a> Platform<'a> {
    fn new(content: &'a mut Vec<Vec<char>>) -> Self {
        Self {
            rotation: -1,
            content: content,
            states: HashMap::new(),
            period: None,
        }
    }

    fn rotate(&mut self) -> Option<(usize, usize)> {
        self.rotation = self.rotation + 1;
        self.roll();
        if self.period.is_none() && self.rotation % 4 == 0 {
            let v = self.as_str();
            if let Some(observed) = self.states.get(&v) {
                self.period = Some((*observed / 4, (self.rotation as usize - *observed) / 4));
            } else {
                self.states.insert(v, self.rotation as usize);
            }
        }
        return self.period;
    }

    fn rows(&self) -> usize {
        if self.rotation % 2 == 0 {
            self.content.len()
        } else {
            self.content[0].len()
        }
    }

    fn cols(&self) -> usize {
        if self.rotation % 2 == 0 {
            self.content[0].len()
        } else {
            self.content.len()
        }
    }

    fn at(&mut self, r: usize, c: usize) -> &mut char {
        let rrows = self.content.len();
        let rcols = self.content[0].len();
        match self.rotation % 4 {
            0 => &mut self.content[r][c],
            1 => &mut self.content[c][r],
            2 => &mut self.content[rrows - 1 - r][c],
            _ => &mut self.content[c][rcols - 1 - r],
        }
    }

    fn roll(&mut self) {
        for c in 0..self.cols() {
            let mut t = 0;
            let mut s = 1;
            while s != self.rows() {
                match (*self.at(t, c), *self.at(s, c)) {
                    ('.', 'O') => {
                        *self.at(t, c) = 'O';
                        *self.at(s, c) = '.';
                        t += 1;
                        s += 1;
                    }
                    (_, 'O') => {
                        s += 1;
                        t = s;
                    }
                    ('.', '.') => {
                        s += 1;
                    }
                    (_, '.') => {
                        t = s;
                        s += 1;
                    }
                    (_, '#') => {
                        s += 1;
                        t = s;
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }

    fn load(&self) -> usize {
        self.content
            .iter()
            .enumerate()
            .map(|(ind, r)| (self.content.len() - ind) * r.iter().filter(|c| **c == 'O').count())
            .sum()
    }

    fn as_str(&self) -> String {
        self.content
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect()
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut content: Vec<Vec<char>> = content
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    let mut platform = Platform::new(&mut content);
    platform.rotate();
    println!("Part 1: {}", platform.load());
    loop {
        if let Some((start, period)) = platform.rotate() {
            let remain = (1000000000 - start) % period;
            for _ in 0..4 * (period + remain) - 1 {
                platform.rotate();
            }
            break;
        }
    }
    println!("Part 2: {}", platform.load());
}
