use std::fs;

struct Mapping {
    source: usize,
    dest: usize,
    length: usize,
}

impl Mapping {
    fn map(&self, value: usize) -> Option<usize> {
        if (self.source..self.source + self.length).contains(&value) {
            return Some(self.dest + (value - self.source));
        }
        return None;
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let mut mappings: Vec<Vec<Mapping>> = vec![];
    let seeds: Vec<usize> = lines[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    for i in 1..lines.len() {
        if lines[i].contains("map") {
            mappings.push(vec![]);
        } else {
            let values: Vec<usize> = lines[i].split(" ").map(|s| s.parse().unwrap()).collect();
            let last = mappings.len() - 1;
            mappings[last].push(Mapping {
                source: values[1],
                dest: values[0],
                length: values[2],
            });
        }
    }
    let mut result: Option<usize> = None;
    for v in seeds {
        print!("{}", v);
        let mut mapped = v;
        for m in &mappings {
            mapped = m
                .into_iter()
                .map(|m| m.map(mapped))
                .filter(|r| r.is_some())
                .find(|_| true)
                .or(Some(Some(mapped)))
                .unwrap()
                .unwrap();
            print!("->{}", mapped);
        }
        println!("");
        if result.is_none() || result.unwrap() > mapped {
            result = Some(mapped);
        }
    }
    println!("Result: {:?}", result);
}
