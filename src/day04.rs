pub const INPUT: &str = include_str!("day04.txt");
type Input = Vec<Vec<char>>;

pub fn run() {
    let input = parse(INPUT);
    println!("Part A: {}", solve_a(input.clone()));
    println!("Part B: {}", solve_b(input));
}

const N: [isize; 2] = [0, -1];
const NE: [isize; 2] = [1, -1];
const E: [isize; 2] = [1, 0];
const SE: [isize; 2] = [1, 1];
const S: [isize; 2] = [0, 1];
const SW: [isize; 2] = [-1, 1];
const W: [isize; 2] = [-1, 0];
const NW: [isize; 2] = [-1, -1];

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn solve_a(input: Input) -> usize {
    let dirs = vec![N, NE, E, SE, S, SW, W, NW];

    let mut total = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'X' {
                for dir in &dirs {
                    let mut valid = true;
                    for i in 1..=3 {
                        if !input
                            .get(((y as isize) + i * dir[1]) as usize)
                            .and_then(|row| row.get(((x as isize) + i * dir[0]) as usize))
                            .is_some_and(|c| c == &XMAS[i as usize])
                        {
                            valid = false;
                            break;
                        }
                    }

                    if valid {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

pub fn solve_b(input: Input) -> usize {
    let mut total = 0;
    for (y, row) in input.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'A' {
                let diag_1: String = [-1, 0, 1]
                    .into_iter()
                    .map(|n: isize| [x as isize + n, y as isize + n])
                    .filter_map(|xy| {
                        (xy[0] >= 0 && xy[1] >= 0).then_some([xy[0] as usize, xy[1] as usize])
                    })
                    .filter_map(|xy| input.get(xy[1]).and_then(|row| row.get(xy[0])))
                    .collect();

                let diag_2: String = [-1, 0, 1]
                    .into_iter()
                    .map(|n| [x as isize + -n, y as isize + n])
                    .filter_map(|xy| {
                        (xy[0] >= 0 && xy[1] >= 0).then_some([xy[0] as usize, xy[1] as usize])
                    })
                    .filter_map(|xy| input.get(xy[1]).and_then(|row| row.get(xy[0])))
                    .collect();

                if (&diag_1 == "MAS" || &diag_1 == "SAM") && (&diag_2 == "MAS" || &diag_2 == "SAM")
                {
                    total += 1;
                }
            }
        }
    }

    total
}

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_A: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    const EXAMPLE_B: &str = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;

    #[test]
    fn part_a() {
        assert_eq!(18, solve_a(parse(EXAMPLE_A)));
        assert_eq!(2642, solve_a(parse(INPUT)));
    }

    #[test]
    fn part_b() {
        assert_eq!(9, solve_b(parse(EXAMPLE_B)));
        assert_eq!(1974, solve_b(parse(INPUT)));
    }
}
