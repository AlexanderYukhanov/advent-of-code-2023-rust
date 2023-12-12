use std::{collections::HashMap, fs};

fn variants(mask: &str, lengths: &[usize], mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if lengths.len() == 0 {
        return 0;
    }
    if let Some(r) = mem.get(&(mask.len(), lengths.len())) {
        return *r;
    }
    let mut result = 0;
    let after: usize = lengths.iter().skip(1).sum::<usize>() + lengths.len() - 1;
    for s in 0..=mask.len() - lengths[0] - after {
        if s > 0 && mask.chars().nth(s - 1).unwrap() == '#' {
            break;
        }
        if after > 0 && mask.chars().nth(s + lengths[0]).unwrap() == '#' {
            continue;
        }
        if mask[s..s + lengths[0]].contains(".") {
            continue;
        }
        result += if lengths.len() == 1 {
            if mask[s + lengths[0]..].contains('#') {
                0
            } else {
                1
            }
        } else {
            variants(&mask[s + lengths[0] + 1..], &lengths[1..], mem)
        }
    }
    mem.insert((mask.len(), lengths.len()), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(variants("..###..", &[2], &mut HashMap::new()), 0);
        assert_eq!(variants("..###..", &[3], &mut HashMap::new()), 1);
    }
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .into_iter()
    {
        if let [mask, lengths, ..] = line.split(" ").collect::<Vec<_>>()[..] {
            let lengths = lengths
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            part1 += variants(mask, &lengths[..], &mut HashMap::new());
            let length = 5 * lengths.len();
            part2 += variants(
                std::iter::repeat(mask)
                    .take(5)
                    .collect::<Vec<_>>()
                    .join("?")
                    .as_str(),
                &lengths
                    .into_iter()
                    .cycle()
                    .take(length)
                    .collect::<Vec<usize>>()[..],
                &mut HashMap::new(),
            );
        } else {
            panic!("Unexpected input: {}", line);
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
