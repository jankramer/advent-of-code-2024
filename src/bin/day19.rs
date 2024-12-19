use aoc::input::Input;
use rustc_hash::FxHashMap;

const IN: Input = Input::new(include_str!("day19.txt"));

fn run(input: Input) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);

    let (left, designs) = input.split_once("\n\n");
    let patterns: Vec<_> = left.split(", ").collect();

    for design in designs.lines() {
        let mut counts: FxHashMap<usize, usize> = FxHashMap::default();
        for prefix in patterns.iter().filter(|&pat| design.starts_with(pat)) {
            *counts.entry(prefix.len()).or_default() += 1;
        }

        if counts.is_empty() {
            continue;
        }

        for i in *counts.keys().min().unwrap()..design.len() {
            let Some(&count) = counts.get(&i) else {
                continue;
            };

            for &pattern in &patterns {
                if (i + pattern.len()) > design.len() || &design[i..i + pattern.len()] != pattern {
                    continue;
                }

                *counts.entry(i + pattern.len()).or_default() += count;
            }
        }

        let Some(count) = counts.get(&design.len()) else {
            continue;
        };

        p1 += 1;
        p2 += count;
    }

    (p1, p2)
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 19\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: Input = Input::new(
        r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#,
    );

    #[test]
    fn test() {
        let test_input_result = run(T1);
        assert_eq!(test_input_result.0, 6);
        assert_eq!(test_input_result.1, 16);

        let real_input_result = run(IN);
        assert_eq!(real_input_result.0, 293);
        assert_eq!(real_input_result.1, 623924810770264);
    }
}
