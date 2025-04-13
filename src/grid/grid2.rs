#![allow(dead_code)]
use std::iter::Peekable;

/// A coordinate in a 2D grid
pub struct Idx((usize, usize));

/// A direction in a 2D grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridDirection {
    South,
    North,
    West,
    East,
}

impl GridDirection {
    fn iter() -> impl Iterator<Item = GridDirection> {
        GridDirectionIter {
            current: GridDirection::North,
            first: GridDirection::North,
            seen_all: false,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            GridDirection::South => GridDirection::West,
            GridDirection::North => GridDirection::East,
            GridDirection::West => GridDirection::North,
            GridDirection::East => GridDirection::South,
        }
    }
}

/// An iterator over grid directions
struct GridDirectionIter {
    current: GridDirection,
    first: GridDirection,
    seen_all: bool,
}

impl Iterator for GridDirectionIter {
    type Item = GridDirection;
    fn next(&mut self) -> Option<Self::Item> {
        if self.seen_all {
            return None;
        }
        let res = match self.current {
            GridDirection::North => GridDirection::East,
            GridDirection::East => GridDirection::South,
            GridDirection::South => GridDirection::West,
            GridDirection::West => GridDirection::North,
        };
        self.current = res.clone();
        if self.current == self.first {
            self.seen_all = true;
        }
        Some(res)
    }
}

/// A 2D grid of values
pub struct Grid2<T> {
    data: Vec<Vec<T>>,
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

    pub fn has_neighbor(&self, idx: (usize, usize), predicate: fn(&T) -> bool) -> bool {
        for dir in GridDirection::iter() {
            let v = self
                .iter_from_index(idx, &dir)
                .skip(1)
                .take(1)
                .collect::<Vec<_>>();
            if let Some(v) = v.first() {
                if predicate(v.1) {
                    return true;
                }
            }
        }
        false
    }

    pub fn index_of_neighbor(&self, idx: (usize, usize), predicate: fn(&T) -> bool) -> Option<Idx> {
        for dir in GridDirection::iter() {
            let v = self
                .iter_from_index(idx, &dir)
                .skip(1)
                .take(1)
                .collect::<Vec<_>>();
            if let Some(v) = v.first() {
                if predicate(v.1) {
                    return Some(Idx(v.0));
                }
            }
        }
        None
    }

    pub fn peek(&self, dir: &GridDirection) -> Option<&T> {
        None
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

    fn new_grid() -> Grid2<char> {
        let string_grid = vec!["123".to_string(), "456".to_string(), "789".to_string()];
        Grid2::from(string_grid)
    }

    #[test]
    fn gird_has_neighbor() {
        let g = new_grid();
        assert!(g.has_neighbor((0, 0), |x| *x == '2'));
        assert!(g.has_neighbor((0, 0), |x| *x == '4'));
        assert!(g.has_neighbor((1, 1), |x| *x == '2'));
        assert!(g.has_neighbor((1, 1), |x| *x == '4'));
        assert!(g.has_neighbor((1, 1), |x| *x == '6'));
        assert!(g.has_neighbor((1, 1), |x| *x == '8'));
        assert!(!g.has_neighbor((1, 1), |x| *x == '9'));
        assert!(!g.has_neighbor((0, 0), |x| *x == '9'));
    }

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

    #[test]
    fn grid_static_dir_iter() {
        let mut it = GridDirection::iter();
        assert_eq!(it.next(), Some(GridDirection::East));
        assert_eq!(it.next(), Some(GridDirection::South));
        assert_eq!(it.next(), Some(GridDirection::West));
        assert_eq!(it.next(), Some(GridDirection::North));
        assert_eq!(it.next(), None);
    }
}
