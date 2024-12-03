use crate::{ProblemData, Solver};

pub struct Day2 {
    data: ProblemData,
}

enum Diff {
    Increasing(i32),
    Decreasing(i32),
    Unsafe(i32),
}

fn to_diff(val: &String) -> Vec<Diff> {
    let vals = val.split_whitespace().filter_map(|y| y.parse::<i32>().ok());
    let next_vals = val
        .split_whitespace()
        .filter_map(|y| y.parse::<i32>().ok())
        .skip(1);

    vals.zip(next_vals)
        .map(|(cur, next)| {
            let diff = next - cur;
            match diff {
                1..=3 => Diff::Decreasing(diff),
                -3..=-1 => Diff::Increasing(diff),
                x => Diff::Unsafe(x),
            }
        })
        .collect::<Vec<_>>()
}

fn to_diff2(val: &String) -> Vec<Diff> {
    let vals = val.split_whitespace().filter_map(|y| y.parse::<i32>().ok());
    let next_vals = val
        .split_whitespace()
        .filter_map(|y| y.parse::<i32>().ok())
        .skip(1);

    let mut have_removed = false;

    vals.zip(next_vals)
        .filter_map(|(cur, next)| {
            let diff = next - cur;
            match diff {
                1..=3 => Some(Diff::Decreasing(diff)),
                -3..=-1 => Some(Diff::Increasing(diff)),
                x => Some(Diff::Unsafe(x)),
            }
        })
        .collect::<Vec<_>>()
}

fn all_same(vec: &Vec<Diff>) -> bool {
    vec.iter().all(|y| matches!(y, Diff::Increasing(..)))
        || vec.iter().all(|z| matches!(z, Diff::Decreasing(..)))
}

// 8 6 4 4 1 -> -2 -2 0 -3
// 8 6 4 1   -> -2 -2 -3 Ok
//
// 1 3 2 4 5 -> 2 -1 -1 -1
// 3 2 4 5   -> -1 -1 -1 Ok
//
// 1 2 7 8 9 -> 1 -5 1 1
// 1 2 8 9   -> 1 -6 1 Failed

fn one_off(vec: &Vec<Diff>) -> bool {
    let x = vec.iter().fold((0, 0, 0), |acc, x| {
        let (inc, dec, inv) = acc;
        match x {
            Diff::Increasing(..) => (inc + 1, dec, inv),
            Diff::Decreasing(..) => (inc, dec + 1, inv),
            Diff::Unsafe(..) => (inc, dec, inv + 1),
        }
    });
    match x {
        (x, 0, 0) => true,
        (x, 1, 0) => true,
        (0, x, 0) => true,
        (1, x, 0) => true,
        _ => false,
    }
    // (x.0 + x.2 == vec.len() || x.1 + x.2 == vec.len()) && matches!(x.2, 0..=1)
}

fn all_inc(x: &Vec<i32>) -> bool {
    x.windows(2)
        .map(|x| x[1] - x[0] > 1 && x[1] - x[0] < 4)
        .all(|x| x == true)
}

fn all_dec(x: &Vec<i32>) -> bool {
    x.windows(2)
        .map(|x| x[0] - x[1] > 1 && x[1] - x[0] < 4)
        .all(|x| x == true)
}

fn variants(x: &Vec<i32>) -> Vec<Vec<i32>> {
    let len = x.len();
    let mut cur = vec![];
    let mut res = vec![];
    // for i in 0..len {
    //     cur.push(
    //         x.iter()
    //             .take(i)
    //             .skip(1)
    //             .take(len - i - 1)
    //             .map(|x| *x)
    //             .collect(),
    //     )
    // }
    res.push(cur);
    println!("{res:?}");
    res
}

fn is_valid(x: &Vec<i32>) -> bool {
    all_inc(x) || all_dec(x) || variants(x).iter().any(|x| all_dec(x) || all_inc(x))
}

impl Solver for Day2 {
    fn new(data: ProblemData) -> Self {
        Self { data }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        self.data.iter().map(to_diff).filter(all_same).count()
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        self.data
            .iter()
            .map(|x| {
                x.split_whitespace()
                    .filter_map(|y| y.parse::<i32>().ok())
                    .collect()
            })
            .filter(|x| is_valid(x))
            .count()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve1() {}
}
