use std::collections::VecDeque;

use crate::{ProblemData, Solver};

pub struct Day3 {
    data: VecDeque<char>,
}

impl Solver for Day3 {
    fn new(data: ProblemData) -> Self {
        Self {
            data: data.iter().flat_map(|c| c.chars()).collect(),
        }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        self.parse().iter().fold(0, |acc, x| match x {
            Operation::Mul(a, b) => acc + (a * b),
            _ => acc,
        })
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        0
    }
}

impl Day3 {
    fn take_expecting(&mut self, expecting: &str) -> bool {
        let len = expecting.len();
        let e: String = self.data.range(..len).cloned().collect();
        if e == expecting {
            self.data.drain(..len);
            return true;
        }
        false
    }

    fn take_number(&mut self) -> Option<i32> {
        let mut nr = vec![];
        while let Some(x) = self.data.get(0) {
            if char::is_digit(*x, 10) {
                nr.push(self.data.pop_front().unwrap());
            } else {
                break;
            }
        }

        return String::from_iter(nr).parse::<i32>().ok();
    }

    fn parse(&mut self) -> Vec<Operation> {
        let mut ops: Vec<Operation> = vec![];
        while let Some(c) = self.data.get(0) {
            let op = match c {
                'm' => {
                    if !self.take_expecting("mul(") {
                        self.data.pop_front();
                        continue;
                    }
                    let Some(a) = self.take_number() else {
                        self.data.pop_front();
                        continue;
                    };
                    if !self.take_expecting(",") {
                        self.data.pop_front();
                        continue;
                    }
                    let Some(b) = self.take_number() else {
                        self.data.pop_front();
                        continue;
                    };
                    if !self.take_expecting(")") {
                        self.data.pop_front();
                        continue;
                    }
                    Operation::Mul(a, b)
                }
                _ => {
                    self.data.pop_front();
                    continue;
                }
            };
            ops.push(op);
        }
        ops
    }
}

struct ParseErr;
#[derive(Debug)]
enum Operation {
    Mul(i32, i32),
    Do,
    Dont,
    Invalid,
}
