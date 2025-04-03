use std::collections::VecDeque;

use crate::{ProblemData, Solver};

pub struct Day3 {
    data: VecDeque<char>,
    ops: Vec<Operation>,
}

impl Solver for Day3 {
    fn new(data: ProblemData) -> Self {
        Self {
            data: data.iter().flat_map(|c| c.chars()).collect(),
            ops: vec![],
        }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        if self.ops.len() == 0 {
            self.ops = self.parse();
        }
        self.ops.iter().fold(0, |acc, x| match x {
            Operation::Mul(a, b) => acc + (a * b),
            _ => acc,
        })
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        if self.ops.len() == 0 {
            self.ops = self.parse();
        }
        let mut state = StateMachine {
            is_enabled: true,
            ops: &self.ops,
        };

        state.run()
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

    fn take_do_or_dont(&mut self) -> Option<Operation> {
        let do_str: String = self.data.range(.."do()".len()).cloned().collect();
        let dont_str: String = self.data.range(.."don't()".len()).cloned().collect();

        if do_str.as_str() == "do()" {
            self.data.drain(..4);
            return Some(Operation::Do);
        } else if dont_str.as_str() == "don't()" {
            self.data.drain(..7);
            return Some(Operation::Dont);
        } else {
            return None;
        }
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
                'd' => {
                    let Some(dodont) = self.take_do_or_dont() else {
                        self.data.pop_front();
                        continue;
                    };
                    dodont
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
struct StateMachine<'a> {
    is_enabled: bool,
    ops: &'a Vec<Operation>,
}

impl<'a> StateMachine<'a> {
    fn run(&mut self) -> i32 {
        let mut acc = 0;
        for x in self.ops {
            acc += match x {
                Operation::Mul(a, b) if self.is_enabled => a * b,
                Operation::Do => {
                    self.is_enabled = true;
                    0
                }
                Operation::Dont => {
                    self.is_enabled = false;
                    0
                }
                _ => 0,
            }
        }
        acc
    }
}
#[derive(Debug)]
enum Operation {
    Mul(i32, i32),
    Do,
    Dont,
}
