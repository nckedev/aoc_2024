use crate::Solver;

pub struct Day17 {
    pub state: State,
    program: Vec<u32>,
}

impl Day17 {
    fn next(&self) -> (Option<Op>, Option<u32>) {
        let a = self.program.get(self.state.pointer).map(|x| Op::from(*x));
        let b = self.program.get(self.state.pointer + 1).map(|x| *x);
        (a, b)
    }

    fn buffer_is_valid(&self) -> bool {
        if self.state.buffer.len() > self.program.len() {
            return false;
        }

        for i in 0..self.state.buffer.len() {
            if self.state.buffer[i] != self.program[i] {
                // println!("{} {}", self.state.buffer[i], self.program[i]);
                return false;
            }
        }

        return true;
    }

    fn reset_and_set_a(&mut self, value: u32) {
        self.state.a = value;
        self.state.buffer = Vec::new();
        self.state.pointer = 0;
    }

    fn execute_op(&mut self, op: Op, operand: u32) {
        match op {
            Op::Adv => {
                self.state.a = Self::div(self.state.a, self.get_combo_operand(operand));
                self.state.pointer += 2;
            }
            Op::Bxl => {
                let b = self.state.b;
                self.state.b = b ^ operand;
                self.state.pointer += 2;
            }
            Op::Bst => {
                self.state.b = self.get_combo_operand(operand) % 8;
                self.state.pointer += 2;
            }
            Op::Jnz => {
                if self.state.a == 0 {
                    self.state.pointer += 2;
                } else {
                    self.state.pointer = operand as usize;
                }
            }
            Op::Bxc => {
                let b = self.state.b;
                self.state.b = b ^ self.state.c;
                self.state.pointer += 2;
            }
            Op::Out => {
                self.state.buffer.push(self.get_combo_operand(operand) % 8);
                self.state.pointer += 2;
            }
            Op::Bdv => {
                self.state.b = Self::div(self.state.a, self.get_combo_operand(operand));
                self.state.pointer += 2;
            }
            Op::Cdv => {
                self.state.c = Self::div(self.state.a, self.get_combo_operand(operand));
                self.state.pointer += 2;
            }
        }
    }

    fn format_output(&self) -> String {
        self.state
            .buffer
            .iter()
            .map(|x| format!("{x}"))
            .zip(std::iter::repeat(','))
            .flat_map(|(a, b)| vec![b.to_string(), a])
            .skip(1)
            .collect::<String>()
    }

    fn run2(&mut self) -> Option<String> {
        while let (Some(op), Some(operand)) = self.next() {
            self.execute_op(op, operand);
            if !self.buffer_is_valid() {
                return None;
            }
        }

        if self.state.buffer.len() != self.program.len() {
            return None;
        }

        Some(self.format_output())
    }

    fn run(&mut self) -> String {
        while let (Some(op), Some(operand)) = self.next() {
            self.execute_op(op, operand);
        }

        self.format_output()
    }
    fn get_combo_operand(&self, operand: u32) -> u32 {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.state.a,
            5 => self.state.b,
            6 => self.state.c,
            7 => panic!("reserved"),
            _ => panic!("invalid operand"),
        }
    }

    fn div(numerator: u32, demominator: u32) -> u32 {
        numerator / 2_u32.pow(demominator)
    }
}

impl Solver for Day17 {
    fn new(data: crate::ProblemData) -> Self {
        let initial_state = data.data[0..=2]
            .iter()
            .map(|x| x.split_whitespace().last().unwrap().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let state = State {
            a: initial_state[0],
            b: initial_state[1],
            c: initial_state[2],
            pointer: 0,
            buffer: Vec::new(),
        };

        let program = data.data[4]
            .split_whitespace()
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        println!("{state:?}");
        println!("{program:?}");
        Self { state, program }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        self.run()
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        let mut i = 1;
        loop {
            if i == 100 {
                return String::from("100");
            }
            self.reset_and_set_a(i);
            if let Some(x) = self.run2() {
                return x;
            }
            i += 1;
        }
    }
}

#[derive(Debug)]
pub struct State {
    a: u32,
    b: u32,
    c: u32,
    pointer: usize,
    buffer: Vec<u32>,
}

enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u32> for Op {
    fn from(value: u32) -> Self {
        match value {
            0 => Op::Adv,
            1 => Op::Bxl,
            2 => Op::Bst,
            3 => Op::Jnz,
            4 => Op::Bxc,
            5 => Op::Out,
            6 => Op::Bdv,
            7 => Op::Cdv,
            _ => panic!("invalid operator"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl State {
        pub fn new(a: u32, b: u32, c: u32) -> Self {
            Self {
                a,
                b,
                c,
                pointer: 0,
                buffer: Vec::new(),
            }
        }
    }

    #[test]
    fn test_example() {
        let mut d = Day17 {
            state: State::new(729, 0, 0),
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let expected = d.run();

        assert_eq!(expected, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn example_1() {
        let mut d = Day17 {
            state: State::new(0, 0, 9),
            program: vec![2, 6],
        };
        let _ = d.run();
        let b = d.state.b;

        assert_eq!(1, b);
    }
    #[test]
    fn example_2() {
        let mut d = Day17 {
            state: State::new(10, 0, 0),
            program: vec![5, 0, 5, 1, 5, 4],
        };
        let r = d.run();

        assert_eq!(r, "0,1,2");
    }
    #[test]
    fn example_3() {
        let mut d = Day17 {
            state: State::new(2024, 0, 0),
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let r = d.run();
        let a = d.state.a;

        assert_eq!(r, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(a, 0);
    }
    #[test]
    fn example_4() {
        let mut d = Day17 {
            state: State::new(0, 29, 0),
            program: vec![1, 7],
        };
        let _ = d.run();
        let b = d.state.b;

        assert_eq!(b, 26);
    }
    #[test]
    fn example_5() {
        let mut d = Day17 {
            state: State::new(0, 2024, 43690),
            program: vec![4, 0],
        };
        let _ = d.run();
        let b = d.state.b;

        assert_eq!(b, 44354);
    }
}
