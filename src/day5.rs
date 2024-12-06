use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::Solver;

pub(crate) struct Day5 {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl Solver for Day5 {
    fn new(data: crate::ProblemData) -> Self {
        let mut rules = Vec::new();
        let mut updates = Vec::new();
        for x in data.data.iter() {
            if x.contains('|') {
                rules.push(Rule::from_str(x).unwrap());
            } else if x.len() == 0 {
                continue;
            } else {
                updates.push(Update::from_str(x).unwrap());
            }
        }
        Self { rules, updates }
    }

    fn solve1(&mut self) -> impl Into<crate::Answer> {
        for u in self.updates.iter_mut() {
            let _ = u.check_order(&self.rules);
        }

        self.updates
            .iter()
            .filter(|x| x.is_valid)
            .fold(0, |acc, x| acc + x.get_center())
    }

    fn solve2(&mut self) -> impl Into<crate::Answer> {
        for u in self.updates.iter_mut() {
            let _ = u.check_and_fix_order(&self.rules);
        }

        self.updates
            .iter()
            .filter(|x| x.is_updated)
            .fold(0, |acc, x| acc + x.get_center())
    }
}

struct Rule {
    first: usize,
    second: usize,
}

#[derive(Debug)]
struct ParseRuleErr;
impl FromStr for Rule {
    type Err = ParseRuleErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut a = s.split('|');
        if let (Some(first), Some(second)) = (a.next(), a.next()) {
            let f = first.parse::<usize>().map_err(|_| ParseRuleErr)?;
            let s = second.parse::<usize>().map_err(|_| ParseRuleErr)?;

            return Ok(Rule {
                first: f,
                second: s,
            });
        }
        Err(ParseRuleErr)
    }
}

struct Update {
    list: Vec<usize>,
    indexed: HashMap<usize, usize>,
    is_valid: bool,
    is_updated: bool,
}

impl Update {
    pub fn get_center(&self) -> usize {
        let middle = self.list.len() / 2;
        *self.list.get(middle).unwrap_or(&0usize)
    }

    fn check_order(&mut self, rules: &Vec<Rule>) {
        for x in rules.iter() {
            let u1 = self.indexed.get(&x.first);
            let u2 = self.indexed.get(&x.second);

            if let (Some(u1), Some(u2)) = (u1, u2) {
                //the index u1 is greater than u2 so it has to be in the wrong spot
                if u1 >= u2 {
                    return;
                }
            }
        }
        self.is_valid = true;
    }

    fn check_and_fix_order(&mut self, rules: &Vec<Rule>) {
        for x in rules.iter() {
            let u1 = self.indexed.get(&x.first).map(|v| *v);
            let u2 = self.indexed.get(&x.second).map(|v| *v);

            if let (Some(u1), Some(u2)) = (u1, u2) {
                //the index u1 is greater than u2 so it has to be in the wrong spot
                if u1 >= u2 {
                    // u1 == index of the first key in the rule

                    //alla indexes som är större eller lika med x.second (u2) ska flyttas +1
                    let mut needs_moving = self
                        .indexed
                        .iter()
                        .filter(|(_, v)| v >= &&u2 && v <= &&u1)
                        .map(|(k, _)| *k)
                        .collect::<Vec<usize>>();

                    // move u2 to u1's place
                    for x in needs_moving.iter_mut() {
                        self.indexed.entry(*x).and_modify(|v| *v += 1);
                    }

                    self.indexed.entry(x.first).and_modify(|v| *v = u2);
                    self.is_updated = true;
                }

                //check that the indexes are still unique
                assert_eq!(
                    self.indexed.keys().len(),
                    HashSet::<&usize>::from_iter(self.indexed.values()).len(),
                    "duplicate indexes !!"
                )
            }
        }
        let mut l = self.indexed.iter().collect::<Vec<(&usize, &usize)>>();
        l.sort_by_key(|x| *x.1);
        self.list = l.iter().map(|(k, _)| **k).collect();
    }
}

#[derive(Debug)]
struct ParseUpdateErr;
impl FromStr for Update {
    type Err = ParseUpdateErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut u = Update {
            list: Vec::with_capacity(s.len()),
            indexed: HashMap::new(),
            is_valid: false,
            is_updated: false,
        };

        for (i, x) in s.split(',').enumerate() {
            let key = x.parse::<usize>().unwrap();
            u.list.push(key);
            u.indexed.entry(key).or_insert(i);
        }
        Ok(u)
    }
}
