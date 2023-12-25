use std::{
    collections::{HashMap, HashSet},
    fs,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Graph = HashMap<String, Vec<String>>;

fn visit(gr: &HashMap<String, Vec<String>>, cut: &HashSet<&String>) -> usize {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut discovered: Vec<&str> = vec![gr.keys().next().unwrap()];
    while let Some(pending) = discovered.pop() {
        if let Some(destinations) = gr.get(pending) {
            for dst in destinations {
                if cut.contains(&edge_to_str(pending, dst)) {
                    continue;
                }
                if visited.insert(&dst) {
                    discovered.push(dst);
                }
            }
        }
    }
    visited.len()
}

fn build_spanning_tree(graph: &Graph, src: &str, exclude: &str) -> Graph {
    let mut visited = HashSet::new();
    let mut spanning_tree = HashMap::new();

    dfs(graph, src, &mut visited, &mut spanning_tree, exclude);

    spanning_tree
}

fn dfs(
    gr: &Graph,
    src: &str,
    visited: &mut HashSet<String>,
    spanning_tree: &mut Graph,
    exclude: &str,
) {
    visited.insert(src.to_string());

    for dst in &gr[src] {
        if !visited.contains(dst) {
            if edge_to_str(dst, src) == exclude {
                continue;
            }
            spanning_tree
                .entry(src.to_string())
                .or_insert_with(Vec::new)
                .push(dst.to_string());
            spanning_tree
                .entry(dst.to_string())
                .or_insert_with(Vec::new)
                .push(src.to_string());

            dfs(gr, dst, visited, spanning_tree, exclude);
        }
    }
}

fn edge_to_str(s: &str, d: &str) -> String {
    if s < d {
        s.to_string() + d
    } else {
        d.to_string() + s
    }
}

fn find_bridges(gr: &Graph, cut1: &str, cut2: &str) -> Vec<(String, String)> {
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();
    let mut entry_time = HashMap::new();
    let mut lowest_time = HashMap::new();
    let mut bridges = Vec::new();
    let mut time = 0;

    dfs2(
        gr.keys().next().unwrap(),
        gr,
        cut1,
        cut2,
        &mut visited,
        &mut parent,
        &mut entry_time,
        &mut lowest_time,
        &mut time,
        &mut bridges,
    );

    bridges
}

fn dfs2(
    src: &str,
    gr: &Graph,
    cut1: &str,
    cut2: &str,
    visited: &mut HashSet<String>,
    parent: &mut HashMap<String, String>,
    entry_time: &mut HashMap<String, usize>,
    lowest_time: &mut HashMap<String, usize>,
    time: &mut usize,
    bridges: &mut Vec<(String, String)>,
) {
    visited.insert(src.to_string());
    *time += 1;
    entry_time.insert(src.to_string(), *time);
    lowest_time.insert(src.to_string(), *time);

    for dst in &gr[src] {
        let edge = edge_to_str(src, dst);
        if edge == cut1 || edge == cut2 {
            continue;
        }
        if !visited.contains(dst) {
            parent.insert(dst.to_string(), src.to_string());
            dfs2(
                dst,
                gr,
                cut1,
                cut2,
                visited,
                parent,
                entry_time,
                lowest_time,
                time,
                bridges,
            );

            if let Some(&lt) = lowest_time.get(&src.to_string()) {
                lowest_time.insert(src.to_string(), lt.min(*lowest_time.get(dst).unwrap()));
            }

            if *lowest_time.get(dst).unwrap() > *entry_time.get(src).unwrap() {
                bridges.push((src.to_string(), dst.to_string()));
            }
        } else if parent.get(src) != Some(dst) {
            lowest_time
                .entry(src.to_string())
                .and_modify(|lt| *lt = (*lt).min(*entry_time.get(dst).unwrap()));
        }
    }
}

fn main() {
    let mut gr: HashMap<String, Vec<String>> = HashMap::new();
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|v| {
            if let Some((src, dsts)) = v.split_once(":") {
                for dst in dsts.trim().split_whitespace() {
                    gr.entry(String::from(src))
                        .or_insert(vec![])
                        .push(String::from(dst));
                    gr.entry(String::from(dst))
                        .or_insert(vec![])
                        .push(String::from(src));
                }
            }
        });
    let spanning_tree = build_spanning_tree(&gr, gr.keys().next().unwrap(), "");
    let edges: HashSet<String> = spanning_tree
        .iter()
        .map(|(src, dsts)| dsts.iter().map(|dst| edge_to_str(src, &dst)))
        .flatten()
        .collect();
    let edges: Vec<String> = edges.into_iter().collect();

    let progress = indicatif::ProgressBar::new(edges.len() as u64);
    for first in &edges {
        progress.inc(1);
        let updated_tree = build_spanning_tree(&gr, gr.keys().next().unwrap(), &first);
        let updated_edges: HashSet<String> = updated_tree
            .iter()
            .map(|(src, dsts)| dsts.iter().map(|dst| edge_to_str(src, &dst)))
            .flatten()
            .collect();
        let updated_edges: Vec<String> = updated_edges.into_iter().collect();
        if let Some(r) = updated_edges
            .into_par_iter()
            .filter_map(|second| {
                let bridges = find_bridges(&gr, first.as_str(), &second.as_str());

                if bridges.len() != 0 {
                    let mut cut: HashSet<&String> = HashSet::new();
                    cut.insert(first);
                    cut.insert(&second);
                    let last_edge = edge_to_str(bridges[0].0.as_str(), bridges[0].1.as_str());
                    cut.insert(&last_edge);
                    let connected = visit(&gr, &cut);
                    Some(connected * (gr.len() - connected))
                } else {
                    None
                }
            })
            .find_any(|_| true)
        {
            progress.finish();
            println!("Part 1: {}", r);
            break;
        }
    }
}
