use std::collections::BTreeMap;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const IN: &str = include_str!("day10.txt");

fn run(input: &str) -> (usize, usize) {
    let grid: BTreeMap<(i64, i64), u32> = input
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(c, h)| h.to_digit(10).map(|h| ((r as i64, c as i64), h)))
        })
        .collect();

    let (mut p1, mut p2) = (0, 0);
    for start in grid.iter().filter(|(_, &h)| h == 0).map(|(p, _)| *p) {
        let trails: FxHashSet<_> = paths(&grid, start).into_iter().collect();
        let peaks: FxHashSet<_> = trails.iter().map(|p| p[0]).collect();

        p1 += peaks.len();
        p2 += trails.len();
    }

    (p1, p2)
}

fn paths(grid: &BTreeMap<(i64, i64), u32>, start: (i64, i64)) -> Vec<Vec<(i64, i64)>> {
    let mut prev: FxHashMap<(i64, i64), FxHashSet<(i64, i64)>> = FxHashMap::default();
    let mut paths = vec![];
    let mut q = vec![start];

    while let Some(p) = q.pop() {
        let prev_h = *grid.get(&p).unwrap();

        for next in nb4(p) {
            let Some(&next_h) = grid.get(&next) else {
                continue;
            };

            if next_h < prev_h || next_h - prev_h != 1 {
                continue;
            }

            prev.entry(next).or_default().insert(p);

            if next_h == 9 {
                paths.extend(reverse(vec![next], &prev));
            }

            q.push(next);
        }
    }

    paths
}

fn reverse(
    path: Vec<(i64, i64)>,
    prev: &FxHashMap<(i64, i64), FxHashSet<(i64, i64)>>,
) -> Vec<Vec<(i64, i64)>> {
    let Some(next) = prev.get(path.last().unwrap()) else {
        return vec![path];
    };

    let mut paths = vec![];
    for node in next {
        paths.extend(reverse(path.iter().chain([node]).cloned().collect(), prev));
    }

    paths
}

fn nb4(p: (i64, i64)) -> Vec<(i64, i64)> {
    vec![
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
        (p.0, p.1 - 1),
        (p.0, p.1 + 1),
    ]
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 01\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test() {
        assert_eq!(run(T1), (36, 81));
    }
}
