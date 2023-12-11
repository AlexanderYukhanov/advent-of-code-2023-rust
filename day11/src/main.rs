use std::{collections::HashSet, fs};

fn analyze(world: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut result = (vec![], vec![]);
    let mut column_empty = vec![true; world[0].len()];
    for (ri, row) in world.iter().enumerate() {
        let mut empty = true;
        for (ci, v) in row.iter().enumerate() {
            if *v != '.' {
                empty = false;
                column_empty[ci] = false;
            }
        }
        if empty {
            result.0.push(ri);
        }
    }
    result.1 = column_empty
        .iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(ci, _)| ci)
        .collect();
    result
}

fn enumerate_galaxies(
    world: &Vec<Vec<char>>,
    empty_rows: &HashSet<usize>,
    empty_columns: &HashSet<usize>,
    speed: usize,
) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut grown_rows = 0;
    for (ri, row) in world.iter().enumerate() {
        if empty_rows.contains(&ri) {
            grown_rows += 1;
        }
        let mut grown_columns = 0;
        for (ci, v) in row.iter().enumerate() {
            if empty_columns.contains(&ci) {
                grown_columns += 1;
            }
            if *v != '.' {
                galaxies.push((
                    ri + grown_rows * (speed - 1),
                    ci + grown_columns * (speed - 1),
                ));
            }
        }
    }
    galaxies
}

fn calculate_distances(galaxies: &Vec<(usize, usize)>) -> u128 {
    let mut sum = 0;
    for f in 1..galaxies.len() {
        for t in 0..f {
            sum += (galaxies[t].0 as i128 - galaxies[f].0 as i128).abs();
            sum += (galaxies[t].1 as i128 - galaxies[f].1 as i128).abs();
        }
    }
    sum as u128
}

fn main() {
    let world: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    let (empty_rows, empty_columns) = analyze(&world);
    let er: HashSet<usize> = empty_rows.into_iter().collect();
    let ec: HashSet<usize> = empty_columns.into_iter().collect();
    println!(
        "Part 1: {}",
        calculate_distances(&enumerate_galaxies(&world, &er, &ec, 2))
    );
    println!(
        "Part 2: {}",
        calculate_distances(&enumerate_galaxies(&world, &er, &ec, 1000000))
    );
}
