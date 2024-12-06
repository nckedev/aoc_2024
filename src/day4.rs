use crate::{grid::grid::Grid, Answer, Solver};

pub struct Day4 {
    grid: Grid<char>,
}

impl Solver for Day4 {
    fn new(data: crate::ProblemData) -> Self {
        Self {
            grid: Grid::from(data.data),
        }
    }

    fn solve1(&mut self) -> impl Into<Answer> {
        let mut acc = 0;
        for row in self.grid.iter_rows() {
            for w in row.windows(4) {
                let s: String = w.iter().map(|&c| *c).collect();
                if s == "XMAS".to_string() || s == "SAMX".to_string() {
                    acc += 1;
                }
            }
        }
        println!("{acc}");
        for row in self.grid.iter_cols() {
            for w in row.windows(4) {
                let s: String = w.iter().map(|&c| *c).collect();
                if s == "XMAS".to_string() || s == "SAMX".to_string() {
                    acc += 1;
                }
            }
        }
        acc
    }

    fn solve2(&mut self) -> impl Into<Answer> {
        0
    }
}
