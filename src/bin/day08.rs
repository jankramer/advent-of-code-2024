use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

const IN: &str = include_str!("day08.txt");

fn run(input: &str) -> (usize, usize) {
    let grid: BTreeMap<(i64, i64), char> = input
        .lines()
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(c, v)| ((r as i64, c as i64), v))
        })
        .collect();

    let width = input.lines().next().unwrap().len() as i64;
    let height = input.lines().count() as i64;

    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();

    let add_if_valid = |foo: &mut HashSet<(i64, i64)>, (r, c): (i64, i64)| {
        (r >= 0 && c >= 0 && r < height && c < width).then(|| foo.insert((r, c)))
    };

    for (_, pts) in grid.into_iter().map(|(k, v)| (v, k)).into_group_map() {
        for (a, b) in pts.into_iter().tuple_combinations() {
            let dr = b.0 - a.0;
            let dc = b.1 - a.1;

            add_if_valid(&mut p1, (a.0 - dr, a.1 - dc));
            add_if_valid(&mut p1, (b.0 + dr, b.1 + dc));

            for i in 0.. {
                if add_if_valid(&mut p2, (a.0 - i * dr, a.1 - i * dc)).is_none() {
                    break;
                };
            }

            for i in 0.. {
                if add_if_valid(&mut p2, (b.0 + i * dr, b.1 + i * dc)).is_none() {
                    break;
                }
            }
        }
    }

    (p1.len(), p2.len())
}

fn main() {
    let (p1, p2) = run(IN);

    println!("Day 01\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    let now = std::time::Instant::now();
    for _ in 0..1000 {
        run(IN);
    }
    let elapsed = now.elapsed();
    println!("{}µs\n", elapsed.as_micros() / 1000);
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (14, 34));
        assert_eq!(run(IN), (369, 1169));
    }
}
