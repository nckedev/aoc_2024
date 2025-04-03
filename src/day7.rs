use crate::Solver;
#[derive(Debug)]
pub struct Day7 {
    tests: Vec<Test>,
}

#[derive(Debug)]
struct Test {
    target: u64,
    numbers: Vec<u64>,
}

impl Solver for Day7 {
    fn new(data: crate::ProblemData) -> Self {
        let tests = data
            .iter()
            .map(|x| {
                if let Some((t, r)) = x.split_once(":") {
                    let t = t.parse::<u64>().expect("cant parse number");
                    let rest = r
                        .split(" ")
                        .filter_map(|x| x.trim().parse::<u64>().ok())
                        .collect::<Vec<u64>>();
                    Test {
                        target: t,
                        numbers: rest,
                    }
                } else {
                    panic!("adf")
                }
            })
            .collect::<Vec<Test>>();

        Self { tests }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        let mut answer = 0;

        for x in self.tests.iter() {
            if can_solve(&x, 0, 0) {
                answer += x.target;
            }
        }
        answer
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        0
    }
}

fn can_solve(t: &Test, idx: usize, current: u64) -> bool {
    if idx == t.numbers.len() {
        if t.target == current {
            // println!("{} {}", t.target, current);
            return true;
        }
        return false;
    }
    let curr_mul = if current == 0 { 1 } else { current };
    can_solve(&t, idx + 1, current + t.numbers[idx])
        || can_solve(&t, idx + 1, curr_mul * t.numbers[idx])
}
