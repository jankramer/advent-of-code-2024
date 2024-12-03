use regex::Regex;

pub const INPUT: &str = include_str!("day03.txt");
type Input = Vec<Instr>;

pub fn run() {
    let now = std::time::Instant::now();
    let input = parse(INPUT);
    println!("{}", now.elapsed().as_micros());
    println!("Part A: {}", solve_a(input.clone()));
    println!("Part B: {}", solve_b(input));
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instr {
    Mul(usize, usize),
    Do,
    Dont,
}

pub fn parse(input: &str) -> Input {
    let regex =
        Regex::new(r#"(?<mul>mul)\((?<a>\d+),(?<b>\d+)\)|(?<do>do)\(\)|(?<dont>don't\(\))"#)
            .unwrap();

    regex
        .captures_iter(input)
        .map(|cap| {
            if cap.name("mul").is_some() {
                return Instr::Mul(
                    cap.name("a").unwrap().as_str().parse().unwrap(),
                    cap.name("b").unwrap().as_str().parse().unwrap(),
                );
            }

            if cap.name("do").is_some() {
                return Instr::Do;
            }

            if cap.name("dont").is_some() {
                return Instr::Dont;
            }

            panic!("unknown instr: {cap:#?}");
        })
        .collect()
}

pub fn solve_a(input: Input) -> usize {
    input
        .iter()
        .map(|instr| match instr {
            Instr::Mul(l, r) => l * r,
            _ => 0,
        })
        .sum()
}

pub fn solve_b(input: Input) -> usize {
    let mut enabled = true;
    let mut total = 0;
    for instr in input {
        match instr {
            Instr::Do => {
                enabled = true;
            }
            Instr::Dont => {
                enabled = false;
            }
            Instr::Mul(l, r) => {
                if enabled {
                    total += l * r;
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    const EXAMPLE_B: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn parses() {
        assert_eq!(
            parse(EXAMPLE_A),
            vec![
                Instr::Mul(2, 4),
                Instr::Mul(5, 5),
                Instr::Mul(11, 8),
                Instr::Mul(8, 5)
            ]
        );

        assert_eq!(
            parse(EXAMPLE_B),
            vec![
                Instr::Mul(2, 4),
                Instr::Dont,
                Instr::Mul(5, 5),
                Instr::Mul(11, 8),
                Instr::Do,
                Instr::Mul(8, 5)
            ]
        )
    }

    #[test]
    fn part_a() {
        assert_eq!(161, solve_a(parse(EXAMPLE_A)));
        assert_eq!(185797128, solve_a(parse(INPUT)));
    }

    #[test]
    fn part_b() {
        assert_eq!(48, solve_b(parse(EXAMPLE_B)));
        assert_eq!(89798695, solve_b(parse(INPUT)));
    }
}
