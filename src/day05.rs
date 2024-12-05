use std::collections::BTreeSet;

pub const INPUT: &str = include_str!("day05.txt");

pub fn run() {
    let input = parse(INPUT);

    println!("Part A: {}", solve_a(&input));
    println!("Part B: {}", solve_b(&input));
}

pub fn solve_a(input: &[Update]) -> usize {
    input
        .iter()
        .filter(|x| x.is_valid())
        .map(|x| x.actual[x.actual.len() / 2])
        .sum()
}

pub fn solve_b(input: &[Update]) -> usize {
    input
        .iter()
        .filter(|x| !x.is_valid())
        .map(|x| x.expected[x.expected.len() / 2])
        .sum()
}

pub fn parse(input: &str) -> Vec<Update> {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules: Vec<_> = rules
        .lines()
        .map(|l| l.split_once('|').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let updates: Vec<Vec<_>> = updates
        .lines()
        .map(|l| l.split(",").map(|c| c.parse().unwrap()).collect())
        .collect();

    updates
        .into_iter()
        .map(|numbers| Update {
            expected: sort(&numbers, &rules),
            actual: numbers,
        })
        .collect()
}

fn sort(numbers: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {
    let mut rules: Vec<&(usize, usize)> = rules
        .iter()
        .filter(|(a, b)| numbers.contains(a) && numbers.contains(b))
        .collect();

    let mut list = vec![];
    let mut xs: BTreeSet<usize> = rules.iter().flat_map(|(a, b)| vec![*a, *b]).collect();

    while !xs.is_empty() {
        let rights: BTreeSet<_> = rules.iter().map(|(_, b)| *b).collect();
        let leftmost = *xs.difference(&rights).next().unwrap();

        list.push(leftmost);
        rules.retain(|(a, _)| a != &leftmost);
        xs.remove(&leftmost);
    }

    list
}

pub struct Update {
    actual: Vec<usize>,
    expected: Vec<usize>,
}

impl Update {
    pub fn is_valid(&self) -> bool {
        self.actual == self.expected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn parses() {
        let input = parse(EXAMPLE);
    }

    #[test]
    fn part_a() {
        assert_eq!(143, solve_a(&parse(EXAMPLE)));
        assert_eq!(5948, solve_a(&parse(INPUT)));
    }

    #[test]
    fn part_b() {
        assert_eq!(123, solve_b(&parse(EXAMPLE)));
        assert_eq!(3062, solve_b(&parse(INPUT)));
    }
}
