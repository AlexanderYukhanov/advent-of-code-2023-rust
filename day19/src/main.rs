use std::{cmp, collections::HashMap, fs};

type Item = Vec<usize>;
type Group = Vec<(usize, usize)>;

struct Condition {
    ind: usize,
    value: usize,
    check: char,
}

impl Condition {
    fn from(s: &str) -> Option<Self> {
        if let Some((cond, _)) = s.split_once(":") {
            let mut chars = cond.chars();
            let ind = match chars.next().unwrap() {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                _ => 3,
            };
            let check = chars.next().unwrap();
            let value = chars.collect::<String>().parse::<usize>().unwrap();
            Some(Self { ind, value, check })
        } else {
            None
        }
    }

    fn pass(&self, item: &Item) -> bool {
        let actual = item[self.ind];
        match self.check {
            '>' => actual > self.value,
            '<' => actual < self.value,
            _ => panic!("Unexpected operation"),
        }
    }

    fn filter(&self, gr: &Group) -> Option<Group> {
        let mut lo = gr[self.ind].0;
        let mut hi = gr[self.ind].1;
        match self.check {
            '>' => lo = cmp::max(lo, self.value + 1),
            '<' => hi = cmp::min(hi, self.value - 1),
            _ => panic!("Unexpected operation"),
        };
        if hi >= lo {
            let mut r = gr.clone();
            r[self.ind] = (lo, hi);
            return Some(r);
        }
        None
    }

    fn remaining(&self, gr: &Group) -> Option<Group> {
        let mut lo = gr[self.ind].0;
        let mut hi = gr[self.ind].1;
        match self.check {
            '>' => hi = cmp::min(hi, self.value),
            '<' => lo = cmp::max(lo, self.value),
            _ => panic!("Unexpected operation"),
        };
        if hi >= lo {
            let mut r = gr.clone();
            r[self.ind] = (lo, hi);
            return Some(r);
        }
        None
    }
}

struct Rule {
    cnd: Option<Condition>,
    dst: String,
}

impl Rule {
    fn from(s: &str) -> Rule {
        if let Some((_, dst)) = s.rsplit_once(":") {
            Rule {
                cnd: Condition::from(s),
                dst: dst.to_string(),
            }
        } else {
            Rule {
                cnd: None,
                dst: s.to_string(),
            }
        }
    }

    fn pass(&self, item: &Item) -> Option<String> {
        if self.cnd.is_none() || self.cnd.as_ref().unwrap().pass(item) {
            Some(self.dst.clone())
        } else {
            None
        }
    }

    fn filter(&self, gr: &Group) -> Option<(String, Group)> {
        if self.cnd.is_none() {
            return Some((self.dst.clone(), gr.clone()));
        }
        if let Some(gr) = self.cnd.as_ref().unwrap().filter(gr) {
            return Some((self.dst.clone(), gr.clone()));
        }
        None
    }

    fn remaining(&self, gr: &Group) -> Option<Group> {
        if self.cnd.is_none() {
            return None;
        }
        return self.cnd.as_ref().unwrap().remaining(gr);
    }
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn from(s: &str) -> Self {
        Self {
            rules: s
                .split(",")
                .map(|s| {
                    if s.ends_with("}") {
                        s.strip_suffix("}").unwrap()
                    } else {
                        s
                    }
                })
                .map(|s| Rule::from(s))
                .collect(),
        }
    }

    fn process(&self, item: &Item) -> String {
        self.rules
            .iter()
            .map(|r| r.pass(item))
            .find(|dst| dst.is_some())
            .unwrap()
            .unwrap()
    }
}

fn read_workflows(lines: &Vec<&str>) -> HashMap<String, Workflow> {
    let mut r: HashMap<String, Workflow> = HashMap::new();
    lines.iter().for_each(|l| {
        if let Some((name, rules)) = l.split_once("{") {
            r.insert(name.to_string(), Workflow::from(rules));
        }
    });
    r
}

fn read_items(lines: &Vec<&str>) -> Vec<Item> {
    lines
        .iter()
        .map(|s| {
            s.strip_prefix("{")
                .and_then(|s| s.strip_suffix("}"))
                .unwrap()
                .split(",")
                .map(|s| {
                    if let Some((_, n)) = s.split_once("=") {
                        n.parse::<usize>().unwrap()
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect()
}

fn accepted(wf: &HashMap<String, Workflow>, item: &Vec<usize>) -> bool {
    let mut flow = String::from("in");
    loop {
        match flow.as_str() {
            "A" => {
                return true;
            }
            "R" => {
                return false;
            }
            _ => {
                flow = wf.get(&flow.to_string()).unwrap().process(item);
            }
        }
    }
}

fn bfs(wfs: &HashMap<String, Workflow>) -> Vec<Group> {
    let mut r = vec![];
    let mut pending = vec![(
        String::from("in"),
        vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
    )];
    while !pending.is_empty() {
        let (dst, gr) = pending.pop().unwrap();
        match dst.as_str() {
            "A" => r.push(gr),
            "R" => {}
            _ => {
                let wf = wfs.get(&dst).unwrap();
                let mut rem = gr;
                for r in wf.rules.iter() {
                    if let Some(n) = r.filter(&rem) {
                        pending.push(n.clone());
                    }
                    if let Some(mrem) = r.remaining(&rem) {
                        rem = mrem;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    r
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content: Vec<&str> = content.split("\n").map(|s| s.trim()).collect();
    let workflows = read_workflows(
        &content
            .iter()
            .take_while(|s| !s.is_empty())
            .map(|s| *s)
            .collect(),
    );
    let iterms = read_items(
        &content
            .iter()
            .skip_while(|s| !s.is_empty())
            .skip(1)
            .map(|s| *s)
            .collect(),
    );

    let part1 = iterms
        .iter()
        .filter(|it| accepted(&workflows, it))
        .map(|it| it.iter().sum::<usize>())
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let passed = bfs(&workflows);
    let part2 = passed
        .iter()
        .map(|gr| {
            gr.iter()
                .map(|(lo, hi)| hi - lo + 1)
                .fold(1_u128, |acc, v| acc * v as u128)
        })
        .sum::<u128>();
    println!("Part 2: {}", part2);
}
