use std::collections::HashMap;
use std::fs;

fn step<'a>(start: &str, dir: char, paths: &'a HashMap<String, (String, String)>) -> &'a str {
    let dsts = paths.get(start).unwrap();
    if dir == 'L' {
        return &dsts.0;
    }
    return &dsts.1;
}

fn find_trip(
    from: &str,
    t: usize,
    paths: &HashMap<String, (String, String)>,
    instructions: &str,
) -> i128 {
    let mut dirs = instructions.chars().cycle().skip(t);
    let mut steps = 0;
    let mut location = from;
    while steps == 0 || !location.ends_with("Z") {
        location = step(location, dirs.next().unwrap(), &paths);
        steps += 1;
    }
    return steps;
}

fn find_trips(
    from: &str,
    paths: &HashMap<String, (String, String)>,
    instructions: &str,
) -> Vec<i128> {
    (0..instructions.len())
        .map(|t| find_trip(from, t, paths, instructions))
        .collect()
}

fn investigate(start: &mut Vec<i128>, trips: &Vec<Vec<i128>>) {
    for g in 0..start.len() {
        let period = trips[g].len() as i128;
        let mut way = 0_i128;
        let initial = start[g] % period;
        let mut ind = initial;
        while way == 0 || ind != initial {
            way += trips[g][ind as usize];
            ind += trips[g][ind as usize];
            ind %= period;
        }
        println!("{}: {} {}", g, start[g], way);
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
    let mut paths: HashMap<String, (String, String)> = HashMap::new();
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let mut dirs = lines[0].chars().cycle();
    lines.iter().skip(1).for_each(|l| {
        let clean: String = l
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ')
            .collect();
        let mut parts = clean.split(" ").filter(|s| !s.is_empty());
        let src = parts.next().unwrap().to_owned();
        paths.insert(
            src,
            (
                parts.next().unwrap().to_owned(),
                parts.next().unwrap().to_owned(),
            ),
        );
    });
    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        current = step(current, dirs.next().unwrap(), &paths);
        steps += 1;
    }
    println!("Part 1: {}", steps);

    /*
    * We cannot brute force the part 2 as there are tooooo many steps.
    * Experiments have shown:
    * 1. each ghost ends up at unique node ending with Z (Z-node)
    * 2. each ghost returns to the same location again and again
    * 3. each ghost revisits the same Z-node after number of steps
    *    required it to the reach the Z-node the original location
    *    (see investigation function).
    * So, all ghosts will simultaneously reach Z-nodes at the number
    * of steps equal to the least common multiple of the number of
    * steps took ghosts to reach the Z-node.
    */
    let ghosts: Vec<&str> = paths
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|v| v.as_str())
        .collect();
    let mut starting: Vec<i128> = vec![];
    let mut trips: Vec<Vec<i128>> = vec![];
    ghosts.iter().for_each(|g| {
        let mut location = *g;
        let mut dirs = lines[0].chars().cycle();
        let mut steps = 0;
        while !location.ends_with("Z") {
            steps += 1;
            location = step(location, dirs.next().unwrap(), &paths);
        }
        println!("{} at {}", location, steps);
        starting.push(steps);
        trips.push(find_trips(location, &paths, lines[0]));
    });
    investigate(&mut starting, &trips);
    println!("Part 2: {}", seq_lcd(&starting));
}
