use crate::input::Input;
use crate::vector::Vec2;
use rustc_hash::FxHashMap;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T = char> {
    pub data: FxHashMap<Vec2, T>,
    pub width: i64,
    pub height: i64,
}

impl<T: Eq> Grid<T> {
    pub fn position(&self, needle: &T) -> Option<Vec2> {
        self.data
            .iter()
            .find(|&(_, val)| val == needle)
            .map(|(pos, _)| *pos)
    }
}

impl Grid<char> {
    pub fn parse(input: &str) -> Grid<char> {
        let width = input.find('\n').unwrap() as i64;
        let height = (input.len() as i64 / (width + 1)) + 1;
        let data = input
            .lines()
            .enumerate()
            .flat_map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(c, chr)| (Vec2(r as i64, c as i64), chr))
            })
            .collect();

        Grid {
            data,
            width,
            height,
        }
    }

    pub fn display(&self) {
        for r in 0..self.height {
            println!(
                "{}",
                (0..self.width)
                    .map(|c| self[Vec2(r, c)])
                    .collect::<String>()
            );
        }
    }
}

impl<T> Index<Vec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: Vec2) -> &Self::Output {
        self.data.get(&index).unwrap()
    }
}

impl<T: Default> IndexMut<Vec2> for Grid<T> {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        self.data.entry(index).or_default()
    }
}

impl From<Input<'_>> for Grid<char> {
    fn from(value: Input) -> Self {
        value.grid()
    }
}

impl From<String> for Grid<char> {
    fn from(value: String) -> Self {
        Grid::parse(&value)
    }
}