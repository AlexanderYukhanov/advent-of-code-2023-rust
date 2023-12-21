use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Signal {
    LOW,
    HIGH,
}

#[derive(Debug)]
struct Pulse {
    from: String,
    to: String,
    signal: Signal,
}

struct Block {
    module: Box<dyn Module>,
    outputs: Vec<String>,
}

struct Broadcast {}

impl Module for Broadcast {
    fn on(&mut self, _: &str, signal: Signal) -> Option<Signal> {
        Some(signal)
    }

    fn add_connection(&mut self, _: &str) {}
}

struct Flipflop {
    on: bool,
}

impl Module for Flipflop {
    fn on(&mut self, _: &str, signal: Signal) -> Option<Signal> {
        if signal == Signal::HIGH {
            return None;
        }
        self.on = !self.on;
        return Some(if self.on { Signal::HIGH } else { Signal::LOW });
    }

    fn add_connection(&mut self, _: &str) {}
}

struct Conjuction {
    mem: HashMap<String, Signal>,
}

impl Module for Conjuction {
    fn on(&mut self, from: &str, signal: Signal) -> Option<Signal> {
        self.mem.insert(from.to_string(), signal);
        return Some(if self.mem.values().all(|s| *s == Signal::HIGH) {
            Signal::LOW
        } else {
            Signal::HIGH
        });
    }

    fn add_connection(&mut self, from: &str) {
        self.mem.insert(from.to_string(), Signal::LOW);
    }
}

trait Module {
    fn on(&mut self, from: &str, signal: Signal) -> Option<Signal>;
    fn add_connection(&mut self, from: &str);
}

struct Commutator {
    blocks: HashMap<String, Block>,
    pending: VecDeque<Pulse>,
    presses: usize,
}

impl Commutator {
    fn push_button(&mut self) -> (usize, usize) {
        self.presses += 1;
        self.pending.push_back(Pulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            signal: Signal::LOW,
        });
        return self.process();
    }

    fn process(&mut self) -> (usize, usize) {
        let mut low = 0_usize;
        let mut high = 0_usize;
        while let Some(pulse) = self.pending.pop_front() {
            if pulse.signal == Signal::LOW {
                low += 1;
            } else {
                high += 1;
            }
            if let Some(block) = self.blocks.get_mut(&pulse.to) {
                if let Some(s) = block.module.on(&pulse.from, pulse.signal) {
                    for o in block.outputs.iter() {
                        self.pending.push_back(Pulse {
                            from: pulse.to.clone(),
                            to: o.clone(),
                            signal: s.clone(),
                        });
                    }
                }
            }
        }
        (low, high)
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcd(a: i128, b: i128) -> i128 {
    (a / gcd(a, b)) * (b / gcd(a, b)) * gcd(a, b)
}

fn seq_lcd(a: &Vec<i128>) -> i128 {
    a.iter().fold(1, |acc, v| lcd(acc, *v))
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut comm = Commutator {
        presses: 0,
        blocks: HashMap::new(),
        pending: VecDeque::new(),
    };

    let mut from: HashMap<String, Vec<String>> = HashMap::new();
    content
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .for_each(|l| {
            if let Some((name, dst)) = l.split_once(" -> ") {
                let block = Block {
                    module: match name.chars().next().unwrap() {
                        '&' => Box::new(Conjuction {
                            mem: HashMap::new(),
                        }),
                        '%' => Box::new(Flipflop { on: false }),
                        _ => Box::new(Broadcast {}),
                    },
                    outputs: dst.split(", ").map(|s| s.to_string()).collect(),
                };
                let name = name.strip_prefix("%").unwrap_or(name);
                let name = name.strip_prefix("&").unwrap_or(name);
                for o in block.outputs.iter() {
                    from.entry(o.clone())
                        .or_insert(vec![])
                        .push(name.to_string());
                }
                comm.blocks.insert(name.to_string(), block);
            }
        });

    for (k, v) in from {
        for f in v {
            comm.blocks.get_mut(&k).and_then(|b| {
                b.module.add_connection(&f);
                Some(b)
            });
        }
    }
    let r = (0..1000)
        .map(|_| comm.push_button())
        .fold((0_usize, 0_usize), |(a, b), (c, d)| (a + c, b + d));
    println!("Part1: {}", r.0 * r.1);
    // just LCD of the periods on the inputs of the last & element
    println!("Part2: {}", seq_lcd(&vec![4049_i128, 3761, 3931, 4079]));
}
