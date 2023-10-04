use std::str::FromStr;

#[allow(dead_code)]
static INPUT: &'static str = include_str!("./day05.txt");

#[derive(Debug)]
enum InstructionError {
    StackDoesNotExist,
    StackEmpty
}

#[derive(Debug)]
struct Instructions {
    #[allow(dead_code)]
    n: usize,
    #[allow(dead_code)]
    from: usize,
    #[allow(dead_code)]
    to: usize
}

impl FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut tokens = s.split(" ");
        let n = tokens.nth(1);
        let from = tokens.nth(1);
        let to = tokens.nth(1);

        if let (Some(n), Some(from), Some(to)) = (
            n.and_then(|x| x.parse::<usize>().ok()),
            from.and_then(|x| x.parse::<usize>().ok()),
            to.and_then(|x| x.parse::<usize>().ok())
        ) {
            Ok(Instructions { n, from: from -1, to: to -1 })
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    #[allow(dead_code)]
    fn execute(&mut self, instruction: &Instructions) -> Result<(), InstructionError> {
        for _ in 0..instruction.n {
            let item = self.0
                .get_mut(instruction.from)
                .ok_or(InstructionError::StackDoesNotExist)?
                .pop()
                .ok_or(InstructionError::StackEmpty)?;

            self.0
                .get_mut(instruction.to)
                .ok_or(InstructionError::StackDoesNotExist)?
                .push(item)
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn execute_while_maintain_order(&mut self, instruction: &Instructions) -> Result<(), InstructionError> {
        
        let stack_of_item = self.0
            .get_mut(instruction.from)
            .ok_or(InstructionError::StackDoesNotExist)?;

        let mut items = stack_of_item.split_off(stack_of_item.len() - instruction.n);

        self.0
            .get_mut(instruction.to)
            .ok_or(InstructionError::StackDoesNotExist)?
            .append(&mut items);
        
        Ok(())
    }

    #[allow(dead_code)]
    fn read_top(&self) -> String {
        self.0.iter().filter_map(|stck| stck.last()).collect()
    }
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Option<(Stacks, Vec<Instructions>)> {
    let mut parts = input.split("\n\n");
    let state_input = parts.next();
    let instructions_input = parts.next();

    if let (Some(state_input), Some(instruction_input)) = (
        state_input, instructions_input
    ) {
        let instructions = instruction_input
            .lines()
            .filter_map(|each_line| {
                each_line.parse::<Instructions>().ok()
            })
            .collect::<Vec<_>>();

        let stack_indices = state_input
            .lines()
            .last()
            .map(|last_line| {
                last_line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| !c.is_whitespace())
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>()
            });
        
        let stacks = stack_indices.map(|stck_idx| {
            stck_idx.iter().copied().map(|i| {
                state_input
                    .lines()
                    .rev()
                    .skip(1)
                    .filter_map(|line| line.chars().nth(i))
                    .filter(|c| !c.is_whitespace())
                    .collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        });

        stacks.map(|stck_vec| (Stacks(stck_vec), instructions))
    } else {
        None
    }
}

#[test]
fn part1() -> Result<(), InstructionError> {
    let (mut stacks, instructions) = parse_input(INPUT).unwrap();

    for instruct in instructions.iter() {
        stacks.execute(instruct)?
    }

    println!("{}", stacks.read_top());

    Ok(())
}

#[test]
fn part2() -> Result<(), InstructionError> {
    let (mut stacks, instructions) = parse_input(INPUT).unwrap();

    for instruct in instructions.iter() {
        stacks.execute_while_maintain_order(instruct)?
    }

    println!("{}", stacks.read_top());

    Ok(())
}