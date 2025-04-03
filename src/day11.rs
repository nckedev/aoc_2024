use crate::Solver;

pub struct Day11 {
    data: Vec<String>,
}

impl Solver for Day11 {
    fn new(data: crate::ProblemData) -> Self {
        if let Some(d) = data.data.first() {
            let s = d.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
            Self { data: s }
        } else {
            panic!("")
        }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        for _ in 0..25 {
            let x = self
                .data
                .iter()
                .map(|a| apply_rule(a.as_str()))
                .flatten()
                .filter(|v| v != "")
                .collect::<Vec<String>>();
            self.data = x;
        }
        self.data.len()
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        ""
    }
}

fn split(d: &str) -> Vec<String> {
    let a = d.split_at(d.len() / 2);
    match a {
        (f, s) if s.chars().all(|c| c == '0') => vec![f.to_string(), "0".to_string()],
        (f, s) if s.starts_with('0') => vec![f.to_string(), s.trim_start_matches('0').to_string()],
        (f, s) => vec![f.to_string(), s.to_string()],
    }
}

pub fn apply_rule(d: &str) -> Vec<String> {
    match d {
        "0" => vec!["1".to_string()],
        x if x.len() % 2 == 0 => split(x),
        x => {
            let y = x.parse::<u64>().expect("not a number");
            vec![(y * 2024).to_string()]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d11_split() {
        let a = "test";
        let b = split(a);
        assert!(b.first().unwrap().len() == 2);
        assert!(b.first().unwrap() == "te");
        assert!(b.last().unwrap() == "st");
    }
    #[test]
    fn d11_split_trim() {
        let a = "1000";
        let b = split(a);
        let (first, second) = (b.first().unwrap(), b.last().unwrap());

        assert!(first == "10");
        assert!(second == "0");
    }
    #[test]
    fn d11_split_trim2() {
        let a = "1001";
        let b = split(a);
        let (first, second) = (b.first().unwrap(), b.last().unwrap());

        assert!(first == "10");
        assert!(second == "1");
    }
}
