fn main() {
    let input = parse(include_str!("input.txt"));
    let test = parse(
        "3   4
4   3
2   5
1   3
3   9
3   3",
    );

    assert_eq!(solve_a(test.clone()), 11);
    println!("Part A: {}", solve_a(input.clone()));

    assert_eq!(solve_b(test), 31);
    println!("Part B: {}", solve_b(input));
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
