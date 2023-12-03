use std::fs;

fn at(l: i32, p: i32, lines: &Vec<&str>) -> char {
    if l < 0 || l as usize >= lines[0].len() || p < 0 || p as usize >= lines.len() {
        return '.';
    }
    return lines[l as usize].chars().nth(p as usize).unwrap();
}

fn good(l: usize, p: usize, lines: &Vec<&str>) -> bool {
    for i in -1 ..= 1 {
        for j in -1 ..= 1 {
            let ch = at((l as i32) + i, (p as i32) + j, lines);
            if ch != '.' && ! ch.is_digit(10) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content.split("\n").collect();
    let mut res = 0;
    let mut cur = 0;
    let mut good_number = false;
    for l in 0..lines.len() {
        if good_number {
            res += cur;
            println!("{}", cur);
        }
        cur = 0;
        good_number = false;
        for i in 0..lines[l].len() {
            if lines[l].chars().nth(i).unwrap().is_digit(10) {
                if good(l, i, &lines) {
                    good_number = true;
                }
                cur = cur * 10 + lines[l].chars().nth(i).unwrap().to_digit(10).unwrap();
            } else {
                if good_number {
                    println!("{}", cur);
                    res += cur;
                }
                cur = 0;
                good_number = false;
            }
        }
    }
    if good_number {
        res += cur;
        println!("{}", cur);
    }
    println!("{}", res);
}
