use std::{collections::HashSet, iter::Peekable};

use crate::{
    grid::grid2::{Grid2, Grid2Iterator, GridDirection},
    Solver,
};
pub(crate) struct Day6 {
    grid: Grid2<Node>,
    map: HashSet<(usize, usize)>,
}

#[derive(PartialEq, Eq)]
pub enum Node {
    Up,
    Down,
    Left,
    Right,
    Free,
    Obs,
}

impl From<char> for Node {
    fn from(x: char) -> Self {
        match x {
            '^' => Self::Up,
            'V' | 'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            '.' => Self::Free,
            '#' => Self::Obs,
            _ => panic!("invalid char"),
        }
    }
}
fn is_start(x: &Node) -> bool {
    match x {
        Node::Up | Node::Down | Node::Left | Node::Right => true,
        _ => false,
    }
}

impl Day6 {
    fn is_heading_off_grid(&self, idx: (usize, usize), dir: &GridDirection) -> bool {
        match (idx, dir) {
            ((0, _), &GridDirection::North) => true,
            ((_, 0), &GridDirection::West) => true,
            ((r, _), &GridDirection::South) if r == self.grid.row_len() - 1 => true,
            ((_, c), &GridDirection::East) if c == self.grid.col_len() - 1 => true,
            _ => false,
        }
    }
}

impl Solver for Day6 {
    fn new(data: crate::ProblemData) -> Self {
        let d = data
            .data
            .iter()
            .map(|s| s.chars().map(|c| Node::from(c)).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();
        Self {
            grid: Grid2::new(d),
            map: HashSet::new(),
        }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        let mut start = (0usize, 0usize);
        let mut dir = GridDirection::South;
        for (row, nodes) in self.grid.iter().enumerate() {
            for (col, n) in nodes.iter().enumerate() {
                if is_start(n) {
                    start = (row, col);
                    dir = GridDirection::from(n);
                    break;
                }
            }
        }
        assert_ne!((0, 0), start);

        let mut pos = Some(start);

        while let Some(p) = pos {
            println!("{p:?}");
            let it = self.grid.iter_from_index(p, &dir);
            for (idx, n) in it {
                if n == &Node::Obs {
                    dir = dir.turn_right();
                    break;
                }
                self.map.insert(idx);
                if self.is_heading_off_grid(idx, &dir) {
                    pos = None;
                } else {
                    pos = Some(idx);
                }
            }
        }
        self.map.len()
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        0
    }
}

pub trait Find<U>
where
    U: PartialEq,
{
    fn find_node(&mut self, pred: fn(&U) -> bool) -> impl Iterator<Item = Option<(usize, usize)>>;
}

// impl<T> Find<T> for Peekable<Grid2Iterator<'_, T>>
// where
//     T: PartialEq + Eq,
// {
//     fn find_node(&mut self, pred: fn(&T) -> bool) -> impl Iterator<Item = <(usize, usize)>> {
//         // TODO: Peek
//
//         // }
//         // for x in self {
//         //     if pred(x.1) {
//         //         let (row, col) = x.0;
//         //         return Some((row, col));
//         //     }
//         // }
//         // None
//     }
// }

impl From<&Node> for GridDirection {
    fn from(value: &Node) -> Self {
        match value {
            Node::Up => GridDirection::North,
            Node::Down => GridDirection::South,
            Node::Left => Self::West,
            Node::Right => Self::East,
            Node::Free => panic!(),
            Node::Obs => panic!(),
        }
    }
}
