use std::collections::{HashMap, HashSet};

use crate::Solver;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Idx {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct IdxOffset(i32, i32);

impl std::ops::Add<IdxOffset> for Idx {
    type Output = Idx;

    fn add(self, rhs: IdxOffset) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl std::ops::Add<IdxOffset> for &Idx {
    type Output = Idx;

    fn add(self, rhs: IdxOffset) -> Self::Output {
        Self::Output {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl std::ops::Sub<IdxOffset> for Idx {
    type Output = Idx;

    fn sub(self, rhs: IdxOffset) -> Self::Output {
        Self {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl std::ops::Sub<IdxOffset> for &Idx {
    type Output = Idx;

    fn sub(self, rhs: IdxOffset) -> Self::Output {
        Self::Output {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl Ord for Idx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y < other.y {
            return std::cmp::Ordering::Less;
        } else if self.y > other.y {
            return std::cmp::Ordering::Greater;
        } else if self.y == other.y && self.x < other.x {
            return std::cmp::Ordering::Less;
        } else if self.y == other.y && self.x > other.x {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }
}

impl PartialOrd for Idx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(i32, i32)> for Idx {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(usize, usize)> for Idx {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as i32,
            y: value.1 as i32,
        }
    }
}

impl Idx {
    fn is_valid<const N: usize>(&self) -> bool {
        self.x >= 0 && self.x < N as i32 && self.y >= 0 && self.y < N as i32
    }

    fn offset_to(&self, other: &Idx) -> IdxOffset {
        IdxOffset(self.x - other.x, self.y - other.y)
    }

    fn next_from_offset<const N: usize>(&self, offset: &IdxOffset) -> Option<Idx> {
        let n = self + *offset;
        if n.is_valid::<N>() {
            return Some(n);
        }
        None
    }

    fn prev_from_offset<const N: usize>(&self, offset: &IdxOffset) -> Option<Idx> {
        let n = self - *offset;
        if n.is_valid::<N>() {
            return Some(n);
        }
        None
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Antenna(char);

impl std::fmt::Display for Antenna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<char> for Antenna {
    fn from(value: char) -> Self {
        Self(value)
    }
}

struct Roof<const N: usize> {
    map: HashMap<Antenna, Vec<Idx>>,
    taken_spots: HashSet<Idx>,
    weakspots: HashSet<Idx>,
}

impl<const N: usize> From<crate::ProblemData> for Roof<N> {
    fn from(value: crate::ProblemData) -> Self {
        let mut s = Self {
            map: HashMap::new(),
            weakspots: HashSet::new(),
            taken_spots: HashSet::new(),
        };
        for (x, r) in value.iter().enumerate() {
            for (y, c) in r.chars().enumerate() {
                if c != '.' {
                    s.map
                        .entry(c.into())
                        .and_modify(|m| m.push(Idx::from((x, y))))
                        .or_insert(vec![Idx::from((x, y))]);
                    s.taken_spots.insert(Idx::from((x, y)));
                }
            }
        }
        s
    }
}

impl<const N: usize> Roof<N> {
    fn map_weakspots(&mut self) {
        for (_, v) in &self.map {
            if v.len() > 1 {
                for first in v {
                    for second in v {
                        if first != second {
                            let offset = first.offset_to(second);
                            let first_w = first + offset;
                            let second_w = second - offset;
                            if Idx::is_valid::<N>(&first_w) {
                                self.weakspots.insert(first_w);
                            }

                            if Idx::is_valid::<N>(&second_w) {
                                self.weakspots.insert(second_w);
                            }
                        }
                    }
                }
            }
        }
    }
    fn map_weakspots2(&mut self) {
        for (_, v) in &self.map {
            if v.len() > 1 {
                for first in v.into_iter() {
                    for second in v {
                        if first != second {
                            let offset = first.offset_to(second);
                            let mut f = *first;

                            while let Some(n) = f.prev_from_offset::<N>(&offset) {
                                self.weakspots.insert(n);
                                f = n;
                            }

                            let mut s = *second;
                            while let Some(p) = s.next_from_offset::<N>(&offset) {
                                self.weakspots.insert(p);
                                s = p;
                            }
                        }
                    }
                }
            }
        }
    }

    fn count_weakspots(&self) -> i32 {
        self.weakspots.len() as i32
    }

    #[allow(dead_code)]
    fn find_antenna_from_idx(&self, idx: &Idx) -> Option<&Antenna> {
        for (k, v) in self.map.iter() {
            if v.iter().any(|x| x == idx) {
                return Some(k);
            }
        }
        None
    }

    #[allow(dead_code)]
    fn pretty_print(&self, show_weakspots: bool) -> () {
        for x in 0..N {
            for y in 0..N {
                let idx = Idx::from((x, y));
                let w = self.weakspots.get(&idx);
                let a = self.find_antenna_from_idx(&idx);
                match (w, a) {
                    (Some(_), None) if show_weakspots => print!("#"),
                    (Some(_), None) => print!("."),
                    (None, Some(a)) => print!("{}", a),
                    (None, None) => print!("."),
                    (Some(_), Some(a)) => print!("{}", a),
                }
            }
            println!();
        }
    }
}

pub struct Day8 {
    roof: Roof<50>,
}

impl Solver for Day8 {
    fn new(data: crate::ProblemData) -> Self {
        let r = Roof::<50>::from(data);
        Self { roof: r }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        self.roof.map_weakspots();
        self.roof.count_weakspots()
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        self.roof.map_weakspots2();
        self.roof.count_weakspots()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_idx_ord_lt() {
        let a = Idx { x: 10, y: 1 };
        let b = Idx { x: 1, y: 10 };

        assert!(a < b)
    }
    #[test]
    fn test_idx_ord_gt() {
        let a = Idx { x: 10, y: 1 };
        let b = Idx { x: 1, y: 10 };

        assert!(b > a)
    }
}
