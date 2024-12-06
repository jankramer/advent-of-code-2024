use std::collections::{BTreeMap, HashSet};

const IN: &str = include_str!("day06.txt");

fn run(input: &str) -> (usize, usize) {
    let grid: BTreeMap<(i64, i64), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| ((y as i64, x as i64), c))
        })
        .collect();

    let (start_pos, _) = grid.iter().find(|(_, &c)| c == '^').unwrap();

    let mut pos = *start_pos;
    let mut dir: (i64, i64) = (-1, 0);

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    visited.insert(pos);

    loop {
        let next = (pos.0 + dir.0, pos.1 + dir.1);

        let Some(next_char) = grid.get(&next).copied() else {
            break;
        };

        if next_char == '#' {
            dir = rotate(dir);
        } else {
            visited.insert(next);
            pos = next;
        }
    }

    let mut obstacles: HashSet<(i64, i64)> = HashSet::new();
    for x in &visited {
        let mut pos = *start_pos;
        let mut dir: (i64, i64) = (-1, 0);

        let mut visited_in_dir: HashSet<((i64, i64), (i64, i64))> = HashSet::new();
        let mut visited2: HashSet<(i64, i64)> = HashSet::new();
        visited2.insert(pos);

        loop {
            let next = (pos.0 + dir.0, pos.1 + dir.1);

            let Some(next_char) = grid.get(&next).copied() else {
                break;
            };

            if &next == x || next_char == '#' {
                dir = rotate(dir);
            } else {
                visited2.insert(next);
                pos = next;
                if !visited_in_dir.insert((pos, dir)) {
                    obstacles.insert(*x);

                    break;
                }
            }
        }
    }

    (visited.len(), obstacles.len())
}

fn rotate(dir: (i64, i64)) -> (i64, i64) {
    match dir {
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        _ => panic!(),
    }
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 06\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (41, 6));
        assert_eq!(run(IN), (5453, 2188));
    }
}
