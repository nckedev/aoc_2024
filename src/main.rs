#![allow(unused_imports)]
use day11::Day11;
use day17::Day17;
use day2::Day2;
use day3::Day3;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;

use day1::Day1;
use std::{fmt::Display, fs};

mod day1;
mod day11;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod graph;
mod grid;
fn main() {
    // let data = ProblemData::from_string("1 2 3 4 5".to_string());
    let data = ProblemData::from_file(8, Case::Real);
    let mut d = Day8::new(data);

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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
impl From<u64> for Answer {
    fn from(value: u64) -> Self {
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

impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}
impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use day4::Day4;
    use rstest::rstest;

    use super::*;

    macro_rules! day {
        ($Foo : ident, $Nr: expr) => {
            &mut $Foo::new(ProblemData::from_file($Nr, Case::Real))
        };
    }
    macro_rules! day_test {
        ($Foo : ident, $Nr: expr) => {
            &mut $Foo::new(ProblemData::from_file($Nr, Case::Test))
        };
    }

    #[rstest]
    #[case(day!(Day1, 1), 3574690)]
    #[case(day!(Day2, 2), 269)]
    #[case(day!(Day3, 3), 187825547)]
    // #[case(day!(Day4, 4), -1)]
    #[case(day!(Day5, 5), 4872)]
    // #[case(day!(Day6, 6), 0)]
    // #[case(day!(Day7, 7), 0)]
    // #[case(day!(Day8, 8), 0)]
    #[case(day!(Day11, 11), 193899)]
    #[case(day!(Day17, 17), "2,3,4,7,5,7,3,0,7")]
    fn solve_part1(#[case] d: &mut impl Solver, #[case] expected: i32) {
        if expected > 0 {
            let p1 = d.solve1();
            assert_eq!(p1.into(), expected.into());
        } else {
            assert!(true);
        }
    }

    #[rstest]
    #[case(day!(Day1, 1), 22565391)]
    #[case(day!(Day2, 2), 0)]
    #[case(day!(Day3, 3), 85508223)]
    #[case(day!(Day4, 4), 0)]
    #[case(day!(Day5, 5), 5564)]
    // #[case(day!(Day6, 6), 0)]
    // #[case(day!(Day7, 7), 0)]
    // #[case(day!(Day8, 8), 0)]
    fn solve_part2(#[case] d: &mut impl Solver, #[case] expected: u32) {
        if expected > 0 {
            let p1 = d.solve2();
            assert_eq!(p1.into(), expected.into());
        }
    }

    #[rstest]
    #[case(day_test!(Day3, 3), 161)]
    #[case(day_test!(Day4, 4), 18)]
    #[case(day_test!(Day5, 5), 143)]
    fn solve_part1_test_case(#[case] d: &mut impl Solver, #[case] expected: u32) {
        if expected > 0 {
            let p1 = d.solve1();
            assert_eq!(p1.into(), expected.into());
        }
    }
    #[rstest]
    #[case(day_test!(Day2, 2), 0)]
    fn solve_part2_test_case(#[case] d: &mut impl Solver, #[case] expected: u32) {
        if expected > 0 {
            let p1 = d.solve2();
            assert_eq!(p1.into(), expected.into());
        }
    }
}
