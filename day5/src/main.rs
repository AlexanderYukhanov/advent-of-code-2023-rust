use std::{cmp, fs};

#[derive(Debug)]
struct Range {
    start: usize,
    length: usize,
}

#[derive(Debug)]
struct Mapping {
    source: usize,
    dest: usize,
    length: usize,
}

#[derive(Debug)]
struct MappingResult {
    mapped: Range,
    remaining: Vec<Range>,
}

impl Mapping {
    fn map(&self, value: usize) -> Option<usize> {
        if (self.source..self.source + self.length).contains(&value) {
            return Some(self.dest + (value - self.source));
        }
        return None;
    }

    fn map_range(&self, range: &Range) -> Option<MappingResult> {
        let start = cmp::max(self.source, range.start);
        let end = cmp::min(self.source + self.length, range.start + range.length) - 1;
        if start > end {
            return None;
        }
        let mut result = MappingResult {
            mapped: Range {
                start: self.dest + (start - self.source),
                length: end - start + 1,
            },
            remaining: vec![],
        };
        if range.start < start {
            result.remaining.push(Range {
                start: range.start,
                length: start - range.start,
            });
        }
        if range.start + range.length > end + 1 {
            result.remaining.push(Range {
                start: end + 1,
                length: range.start + range.length - end - 1,
            });
        }
        return Some(result);
    }
}

fn part1(seeds: &Vec<usize>, mappings: &Vec<Vec<Mapping>>) -> Option<usize> {
    let mut result: Option<usize> = None;
    for seed in seeds {
        let mut mapped = *seed;
        for m in mappings {
            mapped = m
                .into_iter()
                .map(|m| m.map(mapped))
                .filter(|r| r.is_some())
                .find(|_| true)
                .or(Some(Some(mapped)))
                .unwrap()
                .unwrap();
        }
        if result.is_none() || result.unwrap() > mapped {
            result = Some(mapped);
        }
    }
    return result;
}

fn part2(seeds: &Vec<usize>, mappings: &Vec<Vec<Mapping>>) -> Option<usize> {
    let mut ranges: Vec<Range> = seeds
        .windows(2)
        .step_by(2)
        .map(|win| Range {
            start: win[0],
            length: win[1],
        })
        .collect();
    for m in mappings {
        let mut mapped: Vec<Range> = vec![];
        while !ranges.is_empty() {
            let range = ranges.pop().unwrap();
            let maybe_mapped = m
                .into_iter()
                .map(|m| m.map_range(&range))
                .filter(|v| v.is_some())
                .find(|_| true);
            if maybe_mapped.is_some() {
                let mut mapping_result = maybe_mapped.unwrap().unwrap();
                mapped.push(mapping_result.mapped);
                ranges.append(&mut mapping_result.remaining);
            } else {
                mapped.push(range);
            }
        }
        ranges = mapped;
    }
    return ranges.iter().map(|r| r.start).min();
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
    println!("Result: {:?}", part1(&seeds, &mappings));
    println!("Result: {:?}", part2(&seeds, &mappings));
}
