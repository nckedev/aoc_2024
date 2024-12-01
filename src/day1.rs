use std::collections::HashMap;

use crate::{ProblemData, Solver};

pub struct Day1 {
    data: ProblemData,
}

impl Solver for Day1 {
    fn new(data: ProblemData) -> Self {
        Self { data }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];
        for x in self.data.iter() {
            let a = x
                .split_whitespace()
                .map(|b| b.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if let Some(x) = a.get(0) {
                left.push(*x);
            }
            if let Some(x) = a.get(1) {
                right.push(*x);
            }
        }

        assert_eq!(left.len(), right.len());
        left.sort();
        right.sort();

        let b = left
            .iter()
            .zip(right.iter())
            .map(|(&a, &b)| (a - b).abs())
            .fold(0, |acc, diff| acc + diff);

        b
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        let mut left: Vec<i32> = vec![];
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for x in self.data.iter() {
            let a = x
                .split_whitespace()
                .map(|b| b.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            if let Some(x) = a.get(0) {
                left.push(*x);
            }
            if let Some(x) = a.get(1) {
                if let Some(curr) = freq.get(x).or(Some(&0)) {
                    let _ = freq.insert(*x, *curr + 1);
                }
            }
        }

        let mut acc = 0;
        for x in left.iter() {
            if let Some(f) = freq.get(x) {
                acc += x * f;
            }
        }

        acc
    }
}
