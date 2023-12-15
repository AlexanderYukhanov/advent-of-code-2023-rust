use anyhow::{Context, Ok, Result};
use std::{cell::RefCell, collections::HashMap, fs, rc::{Rc, Weak}};

struct LenseNode {
    focus: usize,
    next: Option<Rc<RefCell<LenseNode>>>,
    prev: Option<Rc<RefCell<LenseNode>>>,
}

struct LenseList {
    head: Rc<RefCell<LenseNode>>,
    tail: Rc<RefCell<LenseNode>>,
}

impl LenseList {
    fn new() -> Self {
        let head = Rc::new(RefCell::new(LenseNode {
            focus: 0,
            next: None,
            prev: None,
        }));
        let tail = Rc::new(RefCell::new(LenseNode {
            focus: 0,
            next: None,
            prev: Some(head.clone()),
        }));
        head.borrow_mut().next = Some(tail.clone());
        Self {
            head: head.clone(),
            tail: tail.clone(),
        }
    }

    fn add(&mut self, focus: usize) -> Rc<RefCell<LenseNode>> {
        let node = Rc::new(RefCell::new(LenseNode {
            focus: focus,
            next: Some(self.tail.clone()),
            prev: self.tail.borrow().prev.clone(),
        }));
        if let Some(prev) = self.tail.borrow().prev.clone() {
            prev.borrow_mut().next = Some(node.clone());
        }
        self.tail.borrow_mut().prev = Some(node.clone());
        return node.clone();
    }

    fn remove(&mut self, node: Rc<RefCell<LenseNode>>) {
        if let (Some(prev), Some(next)) = (node.borrow().prev.clone(), node.borrow().next.clone()) {
            prev.borrow_mut().next = Some(next.clone());
            next.borrow_mut().prev = Some(prev.clone());
        }
    }

    fn focusing_power(&self) -> usize {
        let mut result = 0;
        let mut ind = 0;
        let mut current = Some(self.head.clone());
        while let Some(n) = current {
            result += n.borrow().focus * ind;
            ind += 1;
            current = n.borrow().next.clone();
        }
        result
    }
}

struct LenseBox {
    lenses_by_label: HashMap<String, Rc<RefCell<LenseNode>>>,
    lenses: LenseList,
}

impl Default for LenseBox {
    fn default() -> Self {
        LenseBox::new()
    }
}

impl LenseBox {
    fn new() -> Self {
        LenseBox {
            lenses_by_label: HashMap::new(),
            lenses: LenseList::new(),
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(n) = self.lenses_by_label.remove(label) {
            self.lenses.remove(n);
        }
    }

    fn set(&mut self, label: &str, focus: usize) {
        if let Some(n) = self.lenses_by_label.get(label) {
            n.borrow_mut().focus = focus;
        } else {
            self.lenses_by_label
                .insert(String::from(label), self.lenses.add(focus));
        }
    }

    fn focusing_power(&self) -> usize {
        self.lenses.focusing_power()
    }
}

fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0, |acc, c| acc.wrapping_add(c as u8).wrapping_mul(17))
}

fn main() -> Result<()> {
    let content = fs::read_to_string("input.txt").context("reading the input file")?;
    let parts: Vec<&str> = content.split(",").collect();
    let part1: usize = parts.iter().map(|s| hash(s) as usize).sum();
    println!("Part 1: {}", part1);
    let mut boxes: Vec<LenseBox> = (0..256).map(|_| LenseBox::default()).collect();
    parts.iter().for_each(|inst| {
        if let Some((label, focus)) = inst.split_once("=") {
            let ind = hash(label);
            boxes[ind as usize].set(
                label,
                focus.parse().context("parsing focus length").unwrap(),
            );
        } else {
            let label = inst.trim_end_matches("-");
            boxes[hash(label) as usize].remove(label);
        }
    });
    let part2: usize = boxes
        .iter()
        .enumerate()
        .map(|(ind, bx)| (ind + 1) * bx.focusing_power())
        .sum();
    println!("Part 2: {}", part2);

    Ok(())
}
