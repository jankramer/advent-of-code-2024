pub const INPUT: &str = include_str!("day01.txt");
type Input = [[usize; 1000]; 2];

pub fn run() {
    let input = parse(INPUT);
    println!("Part A: {}", solve_a(&input));
    println!("Part B: {}", solve_b(&input));
}

pub fn parse(input: &str) -> Input {
    let mut left = [0; 1000];
    let mut right = [0; 1000];

    let mut iterator = input.split_whitespace().map(|x| x.parse().unwrap());

    for i in 0..1000 {
        left[i] = iterator.next().unwrap_or_default();
        right[i] = iterator.next().unwrap_or_default();
    }

    left.sort_unstable();
    right.sort_unstable();

    [left, right]
}

pub fn solve_a(input: &Input) -> usize {
    input[0]
        .iter()
        .zip(input[1])
        .map(|(x, y)| x.abs_diff(y))
        .sum()
}

pub fn solve_b(input: &Input) -> usize {
    let mut right: [usize; 99999] = [0; 99999];
    for x in input[1] {
        right[x] += 1;
    }

    let mut result = 0;
    for x in input[0] {
        result += x * right[x];
    }

    result
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
        assert_eq!(solve_a(&parse(EXAMPLE)), 11);
        assert_eq!(solve_a(&parse(INPUT)), 2378066);
    }

    #[test]
    fn part_b() {
        assert_eq!(solve_b(&parse(EXAMPLE)), 31);
        assert_eq!(solve_b(&parse(INPUT)), 18934359);
    }
}
