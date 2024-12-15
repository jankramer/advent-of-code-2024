use crate::grid::Grid;
use crate::vector::Vec2;
use std::ops::Deref;

pub struct Input<'a>(&'a str);

impl<'a> Input<'a> {
    pub const fn new(input: &'a str) -> Input<'a> {
        Self(input)
    }

    pub fn split_once(&self, delimiter: &'static str) -> (Input, Input) {
        let (l, r) = self.0.split_once(delimiter).unwrap();

        (l.into(), r.into())
    }

    pub fn grid(&self) -> Grid<char> {
        Grid::parse(self.0)
    }
}

impl Deref for Input<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

pub fn arrow_to_direction(char: char) -> Option<Vec2> {
    match char {
        '^' => Some(Vec2(-1, 0)),
        '>' => Some(Vec2(0, 1)),
        'v' => Some(Vec2(1, 0)),
        '<' => Some(Vec2(0, -1)),
        _ => None,
    }
}
