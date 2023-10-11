use std::collections::VecDeque;

static INPUT: &'static str = include_str!("./day11.txt");

#[derive(Debug, Copy, Clone, Default)]
enum Operation {
    #[default]
    Noop,
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug, Default, Clone)]
pub struct Monkey {
    name: usize,
    items: VecDeque<usize>,
    operations: Operation,
    test_divisor: usize,
    target_true: usize,
    target_false: usize,
    inspected: usize,
}

impl Monkey {
    fn inspect(&mut self) -> Self {
        self.items = self
            .items
            .iter()
            .map(|item| match self.operations {
                Operation::Add(n) => *item + n,
                Operation::Multiply(n) => *item * n,
                Operation::Square => item.pow(2),
                Operation::Noop => *item,
            })
            .collect();
        for _ in 0..self.items.len() {
            self.inspected += 1
        }
        self.to_owned()
    }

    fn bored_monkey(&mut self) -> Self {
        self.items = self
            .items
            .iter()
            .map(|worried_item| *worried_item / 3)
            .collect();
        self.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct VecOfMonkey(Vec<Monkey>);

impl VecOfMonkey {
    fn throw(&mut self) -> Self {
        for _ in 0..20 {
            for i in 0..self.0.len() {
                self.0[i] = self.0[i].inspect().bored_monkey();
                for _ in 0..self.0[i].items.len() {
                    let popped_item = self.0[i].items.pop_front();
                    if let Some(popped) = popped_item {
                        if popped % self.0[i].test_divisor == 0 {
                            let target_true = self.0[i].target_true;
                            self.0[target_true].items.push_back(popped)
                        } else {
                            let target_false = self.0[i].target_false;
                            self.0[target_false].items.push_back(popped)
                        }
                    }
                }
            }
        }
        self.to_owned()
    }

    fn get_max_inspection(&self) -> Vec<usize> {
        let mut vec = self
            .0
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>();
        vec.sort();
        vec.reverse();
        vec
        //vec.iter().take(2).into_iter().sum::<usize>()
    }
}

pub fn parse_input(s: &str) -> VecOfMonkey {
    VecOfMonkey(
        s.split("\n\n")
            .map(|block_of_monkey| {
                let mut monkey = Monkey::default();

                for line in block_of_monkey.lines() {
                    let words = line.trim().split(" ").collect::<Vec<&str>>();

                    match words[0] {
                        "Monkey" => {
                            monkey.name = words[1].strip_suffix(":").unwrap().parse().unwrap()
                        }
                        "Starting" => {
                            let (_, strlist) = line.split_once(": ").unwrap();
                            monkey.items = strlist
                                .split(", ")
                                .map(|w| w.parse::<usize>().unwrap())
                                .collect()
                        }
                        "Operation:" => {
                            monkey.operations = match words[4] {
                                "+" => Operation::Add(words[5].parse().unwrap()),
                                "*" => match words[5] {
                                    "old" => Operation::Square,
                                    n => Operation::Multiply(n.parse().unwrap()),
                                },
                                _ => Operation::Noop,
                            }
                        }
                        "Test:" => monkey.test_divisor = words[3].parse().unwrap(),
                        "If" => {
                            if words[1] == "true:" {
                                monkey.target_true = words[5].parse().unwrap()
                            } else {
                                monkey.target_false = words[5].parse().unwrap()
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                monkey
            })
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        let mut a = parse_input(INPUT);

        let mut b = a.throw().get_max_inspection();

        println!("{:?}", b[0] * b[1]);
    }
}
