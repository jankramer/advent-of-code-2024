const INPUT: &str = include_str!("day01.txt");

pub fn run() {
    println!("Part A: {}", solve_a(parse(INPUT)));
    println!("Part B: {}", solve_b(parse(INPUT)));
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];

    for (a, b) in input.lines().map(|l| l.split_once("   ").unwrap()) {
        left.push(a.parse().unwrap());
        right.push(b.parse().unwrap());
    }

    (left, right)
}

fn solve_a((mut left, mut right): (Vec<usize>, Vec<usize>)) -> usize {
    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn solve_b((left, right): (Vec<usize>, Vec<usize>)) -> usize {
    left.into_iter()
        .map(|x| x * right.iter().filter(|y| **y == x).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn part_a() {
        assert_eq!(solve_a(parse(EXAMPLE)), 11);
        assert_eq!(solve_a(parse(INPUT)), 2378066);
    }

    #[test]
    fn part_b() {
        assert_eq!(solve_b(parse(EXAMPLE)), 31);
        assert_eq!(solve_b(parse(INPUT)), 18934359);
    }
}
