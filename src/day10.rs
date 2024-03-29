static INPUT: &'static str = include_str!("./day10.txt");

#[derive(Debug, Copy, Clone)]
struct Signal {
    cycle: usize,
    strength_during: isize,
    strength_after: isize,
}

impl Signal {
    fn execute_instruction(&mut self, instruction: &Instruction) -> Vec<Self> {
        let mut vec_of_self = vec![];

        for i in 1..=instruction.step {
            self.cycle += 1;
            self.strength_during = self.strength_after;

            if i == 2 {
                self.strength_after += instruction.adder;
            }
            vec_of_self.push(*self);
        }
        vec_of_self
    }

    fn draw_pixel(&self) -> String {
        let current_pixel = ((self.cycle - 1) % 40) as isize;

        let sprite = vec![
            self.strength_during - 1,
            self.strength_during,
            self.strength_during + 1,
        ];

        if self.cycle % 40 == 0 {
            "\n".to_string()
        } else if sprite.contains(&current_pixel) {
            "#".to_string()
        } else {
            ".".to_string()
        }
    }
}

#[derive(Debug)]
struct Instruction {
    idx: usize,
    step: usize,
    adder: isize,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .enumerate() // i used enumerate to help me debug some problem i got
        .map(|(mut idx, line)| {
            idx += 1;
            if line.starts_with("addx") {
                let step = 2 as usize;
                let adder = line.split_ascii_whitespace().nth(1);

                Instruction {
                    idx,
                    step,
                    adder: adder.and_then(|x| x.parse::<isize>().ok()).unwrap(),
                }
            } else {
                let step = 1 as usize;
                let adder = 0 as isize;

                Instruction { idx, step, adder }
            }
        })
        .collect()
}

#[test]
fn part1() {
    let a = parse_input(INPUT);

    let mut signal = Signal {
        cycle: 0,
        strength_during: 1,
        strength_after: 1,
    };

    let b = a
        .iter()
        .map(|instruction| signal.execute_instruction(instruction))
        .flat_map(|sig| sig)
        .collect::<Vec<_>>();

    let c = b
        .iter()
        .skip(19)
        .step_by(40)
        .map(|sig| sig.cycle as isize * sig.strength_during)
        .sum::<isize>();

    println!("{:?}", c);
}

#[test]
fn part2() {
    let a = parse_input(INPUT);

    let mut signal = Signal {
        cycle: 0,
        strength_during: 1,
        strength_after: 1,
    };

    let b = a
        .iter()
        .map(|instruction| signal.execute_instruction(instruction))
        .flat_map(|sig| sig)
        .collect::<Vec<_>>();

    let screen = b.iter().map(|sig| sig.draw_pixel()).collect::<String>();

    println!("{}", screen);
}
