use std::fs;

fn find_starting_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                return (j, i);
            }
        }
    }
    panic!("This code should be unreachable");
}

fn step(start: &(usize, usize), dir: &(i32, i32), map: &Vec<Vec<char>>) -> (i32, i32) {
    let pipe = map[start.1][start.0];
    let res = match (pipe, dir) {
        ('S', _) => *dir,
        ('|', _) => *dir,
        ('-', _) => *dir,
        ('L', (0, 1)) => (1, 0),
        ('L', _) => (0, -1),
        ('J', (0, 1)) => (-1, 0),
        ('J', _) => (0, -1),
        ('7', (0, -1)) => (-1, 0),
        ('7', _) => (0, 1),
        ('F', (0, -1)) => (1, 0),
        ('F', _) => (0, 1),
        _ => panic!("Encountered land"),
    };
    return res;
}

fn main() {
    let mut map: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    let mut on_path = vec![vec![false; map[0].len()]; map.len()];
    let start = find_starting_pos(&map);
    on_path[start.1][start.0] = true;
    let mut pos = start;
    let mut steps = 0;
    let mut dir = (0, -1); // for our case (0, -1) and (1, 0) are valid
    while steps == 0 || pos != start {
        steps += 1;
        dir = step(&pos, &dir, &map);
        pos = (
            (pos.0 as i32 + dir.0) as usize,
            (pos.1 as i32 + dir.1) as usize,
        );
        on_path[pos.1][pos.0] = true;
    }
    println!("Part 1: {}", (steps + 1) / 2);
    
    // do simple scan fill algorithm, the only challenge is '-' lines.
    map[start.1][start.0] = 'L'; // in our case S is equivalent to L
    let mut part2 = 0;
    for i in 0..map.len() {
        let mut outside = true;
        let mut open = ' ';
        for j in 0..map[0].len() {
            if on_path[i][j] && open != ' ' {
                match (open, map[i][j]) {
                    ('F', '7') => open = ' ',
                    ('L', 'J') => open = ' ',
                    (_, '-') => {}
                    _ => {
                        outside = !outside;
                        open = ' ';
                    }
                };
            } else if on_path[i][j] {
                match map[i][j] {
                    'F' => open = 'F',
                    'L' => open = 'L',
                    '|' => {
                        open = ' ';
                        outside = !outside;
                    }
                    _ => {}
                }
            } else {
                map[i][j] = if outside { 'o' } else { 'X' };
                if !outside {
                    part2 += 1;
                }
            }
        }
        for j in 0..map[0].len() {
            print!("{}", map[i][j])
        }
        println!("");
    }
    println!("Part 2: {}", part2);
}
