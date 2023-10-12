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
    pub fn inspect(&mut self) -> Self {
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

    pub fn bored_monkey(&mut self, down_scaler: usize) -> Self {
        self.items = self
            .items
            .iter()
            .map(|worried_item| *worried_item / down_scaler)
            .collect();
        self.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct VecOfMonkey(Vec<Monkey>);

impl VecOfMonkey {
    pub fn throw(&mut self, iteration: usize, divided_by_three: &str) -> Self {
        for _ in 0..iteration {
            for i in 0..self.0.len() {
                if divided_by_three == "yes" {
                    let down_scaler = 3 as usize;
                    self.0[i] = self.0[i].inspect().bored_monkey(down_scaler)
                } else {
                    // 1) the worry value is no longer divided by 3
                    //    but if i proceed with "normal logic", i'll hit arithmetical overflow
                    //    so everytime the monkey inspect and produce new worry value
                    //    i need to scale down the worry value
                    // 2) the most important aspect about worry value is, it will be "tested"
                    //    and moved around to the recipient based on the test
                    //    so, we can use the product of every test_divisor to scale down the worry value
                    //    because eventually they will be tested by the test_divisor

                    let down_scaler = self
                        .0
                        .iter()
                        .map(|monkey| monkey.test_divisor)
                        .product::<usize>();

                    self.0[i] = self.0[i].inspect().bored_monkey(down_scaler)
                }
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

    pub fn get_max_inspection(&self) -> Vec<usize> {
        let mut vec = self
            .0
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>();
        vec.sort();
        vec.reverse();
        vec
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
                            monkey.items = strlist.split(", ").map(|w| w.parse().unwrap()).collect()
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

        let b = a.throw(20, "yes").get_max_inspection();

        println!("{:?}", b[0] * b[1]);
    }

    #[test]
    fn part2() {
        let mut a = parse_input(INPUT);

        let b = a.throw(10_000, "nope").get_max_inspection();

        println!("{:?}", b[0] * b[1]);
    }
}
