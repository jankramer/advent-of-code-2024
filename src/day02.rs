use std::cmp::Ordering;

pub const INPUT: &str = include_str!("day02.txt");
type Input = Vec<Vec<usize>>;

pub fn run() {
    let input = parse(INPUT);
    println!("Part A: {}", solve_a(input.clone()));
    println!("Part B: {}", solve_b(input));
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .collect()
}

pub fn solve_a(input: Input) -> usize {
    input.into_iter().filter(is_safe).count()
}

pub fn solve_b(input: Input) -> usize {
    input.into_iter().filter(is_safe_b).count()
}

fn is_safe(numbers: &Vec<usize>) -> bool {
    let direction = numbers[1].cmp(&numbers[0]);
    if direction == Ordering::Equal {
        return false;
    }

    for i in 0..numbers.len() - 1 {
        if numbers[i + 1].cmp(&numbers[i]) != direction {
            return false;
        }

        if numbers[i].abs_diff(numbers[i + 1]) > 3 {
            return false;
        }
    }

    true
}

fn is_safe_b(numbers: &Vec<usize>) -> bool {
    if is_safe(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let (left, right) = numbers.split_at(i);
        let alt: Vec<_> = left.iter().chain(right.iter().skip(1)).copied().collect();

        if is_safe(&alt) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn parses() {
        let lines = parse(EXAMPLE);
        assert_eq!(lines[0].len(), 5);
        assert_eq!(lines.len(), 6);
    }

    #[test]
    fn part_a() {
        assert_eq!(2, solve_a(parse(EXAMPLE)));
        assert_eq!(486, solve_a(parse(INPUT)));
    }

    #[test]
    fn part_b() {
        assert_eq!(4, solve_b(parse(EXAMPLE)));
        assert_eq!(540, solve_b(parse(INPUT)));
    }
}
