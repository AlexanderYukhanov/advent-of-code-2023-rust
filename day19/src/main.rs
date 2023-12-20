use std::{collections::HashMap, fs};

type Item = Vec<usize>;

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

    let mut part2 = 0_usize;
    for a in 0..=4000 {
        for b in 0..=4000 {
            for c in 0..=4000 {
                for d in 0..=4000 {
                    if accepted(&workflows, &vec![a, b, c, d]) {
                        part2 += a + b + c + d;
                    }
                }
            }
        }
    }
    println!("Part 2: {}", part2);
}
