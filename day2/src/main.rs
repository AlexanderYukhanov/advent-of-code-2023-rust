use std::{cmp, fs, ops::BitOr};

fn allowed_color(color: &str) -> bool {
    let parts: Vec<&str> = color.trim().split(" ").collect();
    let cnt = parts[0].parse::<i32>().unwrap();
    return cnt
        <= match parts[1] {
            "blue" => 14,
            "red" => 12,
            "green" => 13,
            _ => panic!("unexpected color: {}", parts[1]),
        };
}

fn allowed_choice(choice: &str) -> bool {
    choice.trim().split(",").all(allowed_color)
}

fn allowed_game(game: &str) -> bool {
    game.trim()
        .split(':')
        .nth(1)
        .unwrap()
        .split(';')
        .all(allowed_choice)
}

fn weight(game: &str) -> i32 {
    if allowed_game(game) {
        game.split(":")
            .nth(0)
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap()
    } else {
        0
    }
}

struct RGB(i32, i32, i32);

impl RGB {
    fn power(self) -> i32 {
        self.0 * self.1 * self.2
    }
}

impl BitOr for RGB {
    type Output = RGB;
    fn bitor(self, rhs: Self) -> RGB {
        RGB(
            cmp::max(self.0, rhs.0),
            cmp::max(self.1, rhs.1),
            cmp::max(self.2, rhs.2),
        )
    }
}

fn rgb(set: &str) -> RGB {
    let mut rgb = RGB(0, 0, 0);
    for block in set.trim().split(",") {
        let parts: Vec<&str> = block.trim().split(" ").collect();
        let cnt: i32 = parts[0].parse().unwrap();
        match parts[1] {
            "red" => rgb.0 = cnt,
            "green" => rgb.1 = cnt,
            "blue" => rgb.2 = cnt,
            _ => panic!("unexpected color {}", parts[1]),
        }
    }
    return rgb;
}

fn power(game: &str) -> i32 {
    let sets = game.trim().split(":").nth(1).unwrap();
    let mut power = RGB(0, 0, 0);
    for s in sets.split(";") {
        power = power | rgb(s);
    }
    power.power()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", content.split("\n").map(weight).sum::<i32>());
    println!("Part 2: {}", content.split("\n").map(power).sum::<i32>());
}