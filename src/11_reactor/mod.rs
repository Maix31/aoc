use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

struct SeverRack {
    m: HashMap<i32, Vec<i32>>,
}

fn three_lowercase_to_i32(s: &str) -> i32 {
    let bytes = s.as_bytes();
    assert_eq!(bytes.len(), 3, "must be exactly 3 bytes (ASCII letters)");
    assert!(
        bytes.iter().all(|&c| (b'a'..=b'z').contains(&c)),
        "must be [a-z]3"
    );

    let x0 = (bytes[0] - b'a') as i32;
    let x1 = (bytes[1] - b'a') as i32;
    let x2 = (bytes[2] - b'a') as i32;

    x0 * 26 * 26 + x1 * 26 + x2
}

fn parse_input(input: &str) -> SeverRack {
    SeverRack {
        m: input
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(":").unwrap();
                let k = three_lowercase_to_i32(&l);
                let v = r.split_whitespace().map(three_lowercase_to_i32).collect();
                (k, v)
            })
            .collect(),
    }
}

pub fn part_1(input: &str) -> u64 {
    let server_rack = parse_input(input);

    let mut result = 0;
    dfs(
        &server_rack.m,
        three_lowercase_to_i32("you"),
        three_lowercase_to_i32("out"),
        &mut result,
    );

    result
}

fn dfs(m: &HashMap<i32, Vec<i32>>, start: i32, end: i32, result: &mut u64) {
    if let Some(v) = m.get(&start) {
        for child in v {
            if *child == end {
                *result += 1;
            }

            dfs(m, *child, end, result);
        }
    }
}

// pub fn part_2(input: &str) -> u64 {
//     let server_rack = parse_input(input);
//
//     let mut result = 0;
//     dfs_part_2(
//         &server_rack.m,
//         three_lowercase_to_i32("svr"),
//         three_lowercase_to_i32("out"),
//         &mut result,
//         false,
//         false,
//         three_lowercase_to_i32("fft"),
//         three_lowercase_to_i32("dac"),
//         &mut HashSet::new(),
//     );
//
//     result
// }


pub fn part_2(input: &str) -> u64 {
    let server_rack = parse_input(input);

    let svr = three_lowercase_to_i32("svr");
    let out = three_lowercase_to_i32("out");
    let fft = three_lowercase_to_i32("fft");
    let dac = three_lowercase_to_i32("dac");

    let children = match server_rack.m.get(&svr) {
        Some(v) => v,
        None => return 0,
    };

    // Parallelize only the first split (usually enough)
    children
        .par_iter()
        .map(|&child| {
            let mut on_path = HashSet::new();
            // include the start node's effect if you care (svr == fft/dac usually false)
            count_paths_part_2(&server_rack.m, child, out, svr == fft, svr == dac, fft, dac, &mut on_path)
        })
        .sum()
}

fn dfs_part_2(
    m: &HashMap<i32, Vec<i32>>,
    start: i32,
    end: i32,
    result: &mut u64,
    visited_fft: bool,
    visited_dac: bool,
    fft: i32,
    dac: i32,
    on_path: &mut HashSet<i32>,
) {
    if !on_path.insert(start) {
        return;
    }

    let visited_fft = visited_fft || start == fft;
    let visited_dac = visited_dac || start == dac;

    if start == end {
        if visited_fft && visited_dac {
            *result += 1;
        }
        on_path.remove(&start);
        return;
    }

    if let Some(children) = m.get(&start) {
        for &child in children {
            dfs_part_2(
                m,
                child,
                end,
                result,
                visited_fft,
                visited_dac,
                fft,
                dac,
                on_path,
            );
        }
    }

    on_path.remove(&start);
}

fn count_paths_part_2(
    m: &HashMap<i32, Vec<i32>>,
    start: i32,
    end: i32,
    visited_fft: bool,
    visited_dac: bool,
    fft: i32,
    dac: i32,
    on_path: &mut HashSet<i32>,
) -> u64 {
    if !on_path.insert(start) {
        return 0;
    }

    let visited_fft = visited_fft || start == fft;
    let visited_dac = visited_dac || start == dac;

    let mut sum = 0;

    if start == end {
        sum = if visited_fft && visited_dac { 1 } else { 0 };
        on_path.remove(&start);
        return sum;
    }

    if let Some(children) = m.get(&start) {
        for &child in children {
            sum += count_paths_part_2(m, child, end, visited_fft, visited_dac, fft, dac, on_path);
        }
    }

    on_path.remove(&start);
    sum
}

#[cfg(test)]
mod tests;
