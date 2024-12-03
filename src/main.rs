use day2::Day2;
use day3::Day3;

use crate::day1::Day1;
use std::{fmt::Display, fs};

mod day1;
mod day2;
mod day3;
mod grid;
fn main() {
    // let data = ProblemData::from_string("1 2 3 4 5".to_string());
    let data = ProblemData::from_file(3, Case::Real);
    let mut d = Day3::new(data);

    {
        let res1 = d.solve1();
        println!("part1: {}", res1.into());
    }
    {
        let res2 = d.solve2();
        println!("part2: {}", res2.into());
    }
}

trait Solver {
    fn new(data: ProblemData) -> Self;
    fn solve1(&mut self) -> impl Into<Answer>;
    fn solve2(&mut self) -> impl Into<Answer>;
}

// struct Day1 {
//     data: ProblemData,
// }
//
// impl Solver for Day1 {
//     fn new(data: ProblemData) -> Self {
//         Self { data }
//     }
//
//     fn solve1(&mut self) -> impl Into<Answer> {
//         self.data.iter().fold(0, |acc, e| {
//             acc + e.chars().fold(0, |ax, c| {
//                 ax + if let Some(x) = c.to_digit(10) { x } else { 0 }
//             })
//         })
//     }
//
//     fn solve2(&mut self) -> impl Into<Answer> {
//         0
//     }
// }

enum Case {
    Test,
    Real,
}

struct ProblemData {
    data: Vec<String>,
}

impl ProblemData {
    fn iter(&self) -> impl Iterator<Item = &String> {
        self.data.iter()
    }
}

impl ProblemData {
    fn from_file(day: u32, case: Case) -> Self {
        let filename = match case {
            Case::Test => format!("data/day{}_test.txt", day),
            Case::Real => format!("data/day{}.txt", day),
        };

        //read from file
        let content = fs::read_to_string(filename)
            .expect("File not found")
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        Self { data: content }
    }

    fn from_string(data: String) -> Self {
        Self { data: vec![data] }
    }

    fn from_vec(data: Vec<String>) -> Self {
        Self { data }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Answer {
    value: String,
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i32> for Answer {
    fn from(value: i32) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}
impl From<u32> for Answer {
    fn from(value: u32) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day2_part1() {
        let mut d2 = Day2::new(ProblemData::from_file(2, Case::Real));
        assert_eq!(d2.solve1().into(), 269.into())
    }
    #[test]
    fn day2_part2() {
        let mut d2 = Day2::new(ProblemData::from_file(2, Case::Real));
        assert_eq!(d2.solve2().into(), 0.into())
    }
    #[test]
    fn day3_part1_test() {
        let mut d3 = Day3::new(ProblemData::from_file(3, Case::Test));
        assert_eq!(d3.solve1().into(), 0.into())
    }
}
