#![allow(dead_code)]
use std::iter::Peekable;

pub struct Grid2<T> {
    data: Vec<Vec<T>>,
}

#[derive(Debug)]
pub enum GridDirection {
    South,
    North,
    West,
    East,
}

impl GridDirection {
    pub fn turn_right(&self) -> Self {
        match self {
            GridDirection::South => GridDirection::West,
            GridDirection::North => GridDirection::East,
            GridDirection::West => GridDirection::North,
            GridDirection::East => GridDirection::South,
        }
    }
}

impl<T> Grid2<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }
    pub fn iter_from_index<'a>(
        &'a self,
        start: (usize, usize),
        direction: &'a GridDirection,
    ) -> Grid2Iterator<'a, T> {
        Grid2Iterator {
            data: &self.data,
            row: start.0 as i32,
            col: start.1 as i32,
            dir: direction,
        }
    }

    pub fn replace(&mut self, idx: (usize, usize), value: T) -> bool {
        if idx.0 >= self.row_len() || idx.1 >= self.col_len() {
            return false;
        }

        self.data[idx.0][idx.1] = value;
        return true;
    }

    pub fn row_len(&self) -> usize {
        self.data.len()
    }

    pub fn col_len(&self) -> usize {
        self.data[0].len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.data.iter()
    }
    // fn movedir(dir: &GridDirection, col: usize) -> usize {
    //     match dir {
    //         GridDirection::West => col - 1,
    //         GridDirection::East => col + 1,
    //         _ => panic!("not a valid direction"),
    //     }
    // }
}

impl From<Vec<String>> for Grid2<char> {
    fn from(value: Vec<String>) -> Self {
        let r: Vec<Vec<char>> = value.iter().map(|x| x.chars().collect()).collect();
        Self { data: r }
    }
}

#[derive(Debug)]
pub(crate) struct Grid2Iterator<'a, T> {
    data: &'a Vec<Vec<T>>,
    row: i32,
    col: i32,
    dir: &'a GridDirection,
}

impl<'a, T> Iterator for Grid2Iterator<'a, T> {
    type Item = ((usize, usize), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.data.len() as i32 || self.col >= self.data[0].len() as i32 {
            return None;
        }
        if self.row < 0 || self.col < 0 {
            return None;
        }
        let r = self.row as usize;
        let c = self.col as usize;
        let res = Some(((r, c), &self.data[r][c]));

        match self.dir {
            GridDirection::North => self.row -= 1,
            GridDirection::South => self.row += 1,
            GridDirection::West => self.col -= 1,
            GridDirection::East => self.col += 1,
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_row() {
        let strs = ["123", "456"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        let g = Grid2::from(strs);

        let mut it = g.iter_from_index((1, 0), &GridDirection::East);

        assert_eq!(it.next(), Some(((1, 0), &'4')));
        assert_eq!(it.next(), Some(((1, 1), &'5')));
        assert_eq!(it.next(), Some(((1, 2), &'6')));
        assert_eq!(it.next(), None);
    }
    #[test]
    fn iter_col() {
        let strs = ["123", "456"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        let g = Grid2::from(strs);

        let mut it = g.iter_from_index((0, 1), &GridDirection::South);

        // assert_eq!(it.next(), Some(((1, 0), &'4')));
        assert_eq!(it.next(), Some(((0, 1), &'2')));
        assert_eq!(it.next(), Some(((1, 1), &'5')));
        assert_eq!(it.next(), None);
    }
}
