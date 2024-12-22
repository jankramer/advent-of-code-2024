use aoc::input::Input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::VecDeque;

const IN: Input = Input::new(include_str!("day21.txt"));

fn run(input: Input) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);

    let nbs_num = FxHashMap::from_iter(vec![
        ('A', vec![('<', '0'), ('^', '3')]),
        ('0', vec![('^', '2'), ('>', 'A')]),
        ('1', vec![('>', '2'), ('^', '4')]),
        ('2', vec![('<', '1'), ('^', '5'), ('>', '3'), ('v', '0')]),
        ('3', vec![('<', '2'), ('^', '6'), ('v', 'A')]),
        ('4', vec![('>', '5'), ('^', '7'), ('v', '1')]),
        ('5', vec![('<', '4'), ('^', '8'), ('>', '6'), ('v', '2')]),
        ('6', vec![('<', '5'), ('^', '9'), ('v', '3')]),
        ('7', vec![('>', '8'), ('v', '4')]),
        ('8', vec![('<', '7'), ('v', '5'), ('>', '9')]),
        ('9', vec![('<', '8'), ('v', '6')]),
    ]);

    let nbs_dir = FxHashMap::from_iter(vec![
        ('^', vec![('>', 'A'), ('v', 'v')]),
        ('A', vec![('<', '^'), ('v', '>')]),
        ('<', vec![('>', 'v')]),
        ('v', vec![('<', '<'), ('^', '^'), ('>', '>')]),
        ('>', vec![('<', 'v'), ('^', 'A')]),
    ]);

    let paths_num: FxHashMap<(char, char), Vec<Path>> = Path::build_lookup(nbs_num);
    let paths_dir: FxHashMap<(char, char), Vec<Path>> = Path::build_lookup(nbs_dir);

    for line in input.lines() {
        let code = line.trim_end_matches('A').parse::<usize>().unwrap();
        let path: Path = line.chars().collect::<Vec<_>>().into();

        let paths = path.expand(&paths_num);
        let mut paths: Vec<Path> = paths.into_iter().map_into().collect();

        for i in 0..25 {
            let mut new_paths = vec![];
            for path in paths {
                new_paths.extend(path.expand(&paths_dir));
            }

            new_paths.sort();
            new_paths.truncate(100);

            paths = new_paths;

            if i == 1 {
                p1 += code * paths.iter().map(|p| p.len()).min().unwrap();
            }
        }

        p2 += code * paths.iter().map(|p| p.len()).min().unwrap();
    }

    (p1, p2)
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Path {
    counts: FxHashMap<(char, char), usize>,
}

impl Path {
    fn build_lookup(
        neighbors: FxHashMap<char, Vec<(char, char)>>,
    ) -> FxHashMap<(char, char), Vec<Self>> {
        neighbors
            .keys()
            .flat_map(|&start| paths(&neighbors, start))
            .map(|(k, v)| (k, v.into_iter().map_into().collect()))
            .collect()
    }

    fn len(&self) -> usize {
        self.counts.values().sum()
    }

    fn num_same(&self) -> usize {
        self.counts
            .iter()
            .filter_map(|((a, b), c)| (a == b).then_some(*c))
            .sum()
    }

    fn expand(self, lookup: &FxHashMap<(char, char), Vec<Path>>) -> Vec<Self> {
        let mut result: Vec<Self> = vec![Default::default()];

        for (pair, count) in self.counts {
            let mut new_result = vec![];
            for new_path in &lookup[&pair] {
                for current in &result {
                    new_result.push(current.merge(new_path, count));
                }
            }
            result = new_result;
        }

        result
    }

    fn merge(&self, other: &Path, multiplier: usize) -> Self {
        let mut counts = self.counts.clone();
        for (pair, count) in other.counts.iter() {
            *counts.entry(*pair).or_default() += count * multiplier;
        }

        Self { counts }
    }
}

impl From<Vec<char>> for Path {
    fn from(value: Vec<char>) -> Self {
        let mut counts = FxHashMap::default();
        for (a, b) in ['A'].into_iter().chain(value.into_iter()).tuple_windows() {
            *counts.entry((a, b)).or_default() += 1;
        }

        Self { counts }
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len()
            .cmp(&other.len())
            .then_with(|| self.num_same().cmp(&other.num_same()).reverse())
    }
}

fn paths(
    nbs: &FxHashMap<char, Vec<(char, char)>>,
    start: char,
) -> FxHashMap<(char, char), Vec<Vec<char>>> {
    let mut dist: FxHashMap<char, usize> = FxHashMap::default();
    let mut prev: FxHashMap<char, FxHashSet<(char, char)>> = Default::default();
    let mut q = VecDeque::from([(start, 0)]);
    while let Some((current, curr_dist)) = q.pop_front() {
        for &(key, val) in nbs.get(&current).unwrap() {
            if curr_dist > dist.get(&val).copied().unwrap_or(usize::MAX) {
                continue;
            }

            prev.entry(val).or_default().insert((key, current));
            dist.insert(val, curr_dist + 1);
            q.push_back((val, curr_dist + 1));
        }
    }

    nbs.keys()
        .map(|&end| {
            let mut paths = reverse(&prev, start, end);
            for path in paths.iter_mut() {
                path.push('A');
            }

            ((start, end), paths)
        })
        .collect()
}

fn reverse(
    prev: &FxHashMap<char, FxHashSet<(char, char)>>,
    start: char,
    end: char,
) -> Vec<Vec<char>> {
    if start == end {
        return vec![vec![]];
    }

    let mut paths = vec![];
    for &(key, val) in prev.get(&end).unwrap() {
        for mut path in reverse(prev, start, val) {
            path.push(key);
            paths.push(path);
        }
    }

    paths
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

    const T1: Input = Input::new(
        r#"029A
980A
179A
456A
379A"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 126384);
        // assert_eq!(test_input_result.1, 0);

        // let real_input_result = run(IN);
        // assert_eq!(real_input_result.0, 0);
        // assert_eq!(real_input_result.1, 0);
    }
}
