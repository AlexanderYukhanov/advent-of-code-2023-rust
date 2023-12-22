use std::time::Duration;
use std::{collections::HashSet, fs};

use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::pixels::Color;
use sdl2::rect::Point;

fn starting_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                return (j, i);
            }
        }
    }
    return (0, 0);
}

fn remap(pos: (i32, i32), w: i32, h: i32) -> (usize, usize) {
    (
        if pos.0 > 0 {
            (pos.0 % w) as usize
        } else {
            ((pos.0 % w + w) % w) as usize
        },
        if pos.1 > 0 {
            (pos.1 % h) as usize
        } else {
            ((pos.1 % h + h) % h) as usize
        },
    )
}

fn part1(map: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> usize {
    let mut pending: HashSet<(i32, i32)> = HashSet::new();
    pending.insert((start.0 as i32, start.1 as i32));
    for _ in 0..steps {
        let mut nxt: HashSet<(i32, i32)> = HashSet::new();
        for dt in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
            for s in &pending {
                if let Some(c) = map
                    .get((s.1 + dt.1) as usize)
                    .and_then(|l| l.get((s.0 + dt.0) as usize))
                {
                    if *c == '.' || *c == 'S' {
                        nxt.insert((s.0 + dt.0, s.1 + dt.1));
                    }
                }
            }
        }
        pending = nxt;
    }
    return pending.len();
}

fn part2(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    steps: usize,
    canvas: &mut Canvas<Window>,
) -> usize {
    let mut pending: HashSet<(i32, i32)> = HashSet::new();
    pending.insert((start.0 as i32, start.1 as i32));
    for step in 1..=steps {
        let mut nxt: HashSet<(i32, i32)> = HashSet::new();
        for dt in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
            for s in &pending {
                let pos = (s.0 + dt.0, s.1 + dt.1);
                let mapped = remap(pos, map.len() as i32, map[0].len() as i32);
                let c = map[mapped.1][mapped.0];
                if c == '.' || c == 'S' {
                    nxt.insert(pos);
                }
            }
        }
        pending = nxt;
        if step % 131 == 65 {
            print!("Steps: {} Opened: {} ", step, pending.len());
            let mut inside = 0;
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            for (x, y) in &pending {
                canvas.set_draw_color(Color::RGB(100, 100, 100));

                if (x.abs() % 131 - 65).abs() + (y.abs() % 131 - 65).abs() <= 65 {
                    inside += 1;
                    canvas.set_draw_color(Color::RGB(255, 0, 255));
                }

                canvas
                    .draw_point(Point::new(1280 / 2 + x, 1024 / 2 + y))
                    .unwrap();
            }
            println!("Inside: {}", inside);
            canvas.present();
            std::thread::sleep(Duration::from_secs(0));
        }
    }

    return pending.len();
}

fn vacant(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .map(|l| l.iter().filter(|c| **c != '#').count())
        .sum::<usize>()
}

fn main() {
    let content: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    let start = starting_pos(&content);

    println!("Part1: {}", part1(&content, start, 64));
    let n = ((26501365- start.0) / content.len()) as u64;
    println!("Part2: {}", (n + 1) * (n + 1) * 3734 + n*n*3615 + n*(n+1) * 7367);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("explore", 1280, 1024)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    part2(&content, start, 1000, &mut canvas);
    println!("Size: {}", vacant(&content));
}
