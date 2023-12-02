use std::{cmp, fs, ops::BitOr};

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

fn game_power_rgb(game: &str) -> RGB {
    let sets = game.trim().split(":").nth(1).unwrap();
    let mut power_rgb = RGB(0, 0, 0);
    for s in sets.split(";") {
        power_rgb = power_rgb | rgb(s);
    }
    power_rgb
}

fn possible_game(game: &str) -> bool {
    let rgb = game_power_rgb(game);
    rgb.0 <= 12 && rgb.1 <= 13 && rgb.2 <= 14
}

fn weight(game: &str) -> i32 {
    if possible_game(game) {
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

fn power(game: &str) -> i32 {
    game_power_rgb(game).power()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", content.split("\n").map(weight).sum::<i32>());
    println!("Part 2: {}", content.split("\n").map(power).sum::<i32>());
}