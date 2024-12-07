const IN: &str = include_str!("day07.txt");

fn run(input: &str) -> (i64, i64) {
    let (mut p1, mut p2) = (0, 0);

    for line in input.lines() {
        let (ans, nums) = line.split_once(": ").unwrap();
        let ans: i64 = ans.parse().unwrap();
        let xs: Vec<_> = nums
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let mut ys_p1 = vec![xs[0]];
        let mut ys_p2 = vec![xs[0]];
        for x in xs.into_iter().skip(1) {
            let mut ys_p1_new = vec![];
            let mut ys_p2_new = vec![];

            for y in ys_p1 {
                if x + y <= ans {
                    ys_p1_new.push(x + y);
                }

                if x * y <= ans {
                    ys_p1_new.push(x * y);
                }
            }

            for y in ys_p2 {
                if x + y <= ans {
                    ys_p2_new.push(x + y);
                }

                if x * y <= ans {
                    ys_p2_new.push(x * y);
                }

                let concat = y * 10i64.pow(x.ilog10() + 1) + x;
                if concat <= ans {
                    ys_p2_new.push(concat);
                }
            }

            ys_p1 = ys_p1_new;
            ys_p2 = ys_p2_new;
        }

        if ys_p1.into_iter().any(|x| x == ans) {
            p1 += ans;
            p2 += ans;
            continue;
        }

        if ys_p2.into_iter().any(|x| x == ans) {
            p2 += ans;
        }
    }

    (p1, p2)
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 07\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test() {
        assert_eq!(run(T1), (3749, 11387));
        assert_eq!(run(IN), (2664460013123, 426214131924213));
    }
}
