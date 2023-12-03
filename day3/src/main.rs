use std::{fs, collections::{HashMap, HashSet}};

fn get_char_at(l: i32, p: i32, lines: &Vec<&str>) -> char {
    if l < 0 || l as usize >= lines[0].len() || p < 0 || p as usize >= lines.len() {
        return '.';
    }
    return lines[l as usize].chars().nth(p as usize).unwrap();
}

fn has_symbol_neighbor(l: usize, p: usize, lines: &Vec<&str>) -> bool {
    for i in -1 ..= 1 {
        for j in -1 ..= 1 {
            let ch = get_char_at((l as i32) + i, (p as i32) + j, lines);
            if ch != '.' && ! ch.is_digit(10) {
                return true;
            }
        }
    }
    return false;
}

fn collect_starts(l: usize, p: usize, nid: usize, lines: &Vec<&str>, stars: &mut HashMap<(usize, usize), HashSet<usize>>) {
    for i in -1 ..= 1 {
        for j in -1 ..= 1 {
            let ch = get_char_at((l as i32) + i, (p as i32) + j, lines);
            if ch == '*' {
                let starpos = ((l as i32 + i) as usize, (p as i32 + j) as usize);
                stars.entry(starpos).or_insert(HashSet::new()).insert(nid);
            }
        }
    }
}

fn part1(lines: &Vec<&str>) -> u32 {
    let mut res = 0;
    let mut cur = 0;
    let mut good = false;
    for l in 0..lines.len() {
        if good {
            res += cur;
        }
        cur = 0;
        good = false;
        for i in 0..lines[l].len() {
            if lines[l].chars().nth(i).unwrap().is_digit(10) {
                if has_symbol_neighbor(l, i, &lines) {
                    good = true;
                }
                cur = cur * 10 + lines[l].chars().nth(i).unwrap().to_digit(10).unwrap();
            } else {
                if good {
                    res += cur;
                }
                cur = 0;
                good = false;
            }
        }
    }
    if good {
        res += cur;
    }
    return res;
}

fn part2(lines: &Vec<&str>) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    let mut stars: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
    let mut cur = 0;
    for l in 0..lines.len() {
        for i in 0..lines[l].len() {
            if lines[l].chars().nth(i).unwrap().is_digit(10) {
                collect_starts(l, i, numbers.len(), &lines, &mut stars);
                cur = cur * 10 + lines[l].chars().nth(i).unwrap().to_digit(10).unwrap();
            } else {
                if cur != 0 {
                    numbers.push(cur);
                    cur = 0;
                }
            }
        }
        if cur != 0 {
            numbers.push(cur);
            cur = 0;
        }
    }
    let  mut res = 0;
    for key in stars.keys() {
        let nmb = &stars[key];
        if nmb.len() == 2 {
            res += nmb.iter().map(|ind| numbers[*ind]).fold(1, |acc, v| acc * v)
        }
    }
    return res;
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content.split("\n").collect();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}