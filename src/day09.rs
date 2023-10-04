use std::collections::HashSet;

static INPUT: &'static str = include_str!("./day09.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Knot(i32, i32);

impl Knot {
    fn mutate_field(&mut self, dx: i32, dy: i32) {
        self.0 += dx;
        self.1 += dy;
    }

    fn add_signum(&mut self, x_dif: i32, y_dif: i32) {
        self.0 += x_dif.signum();
        self.1 += y_dif.signum();
    }
}

#[derive(Debug)]
struct Rope(Vec<Knot>);

impl Rope {
    fn new(knot: Knot, tail_amount: usize) -> Self {
        let new_vec = vec![knot; tail_amount + 1];

        Self(new_vec)
    }

    fn tail_movement(&mut self, motion: &Motions) -> Vec<Knot> {
        let mut visited = Vec::new();
        let (dx, dy) = match motion.direction {
            "D" => (0, 1),
            "U" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..motion.step {
            self.0[0].mutate_field(dx, dy);

            for i in 1..self.0.len() {
                let x_dif = self.0[i - 1].0 - self.0[i].0;
                let y_dif = self.0[i - 1].1 - self.0[i].1;

                if x_dif.abs() > 1 || y_dif.abs() > 1 {
                    self.0[i].add_signum(x_dif, y_dif);
                }
            }
            visited.push(self.0[self.0.len() - 1]);
        }

        visited
    }
}

#[derive(Debug)]
struct Motions<'a> {
    direction: &'a str,
    step: u8,
}

fn parse_input(input: &str) -> Vec<Motions> {
    input
        .lines()
        .filter_map(|line| {
            let mut tokens = line.split_ascii_whitespace();
            let direction = tokens.next();
            let step = tokens.next();

            if let (Some(direction), Some(step)) =
                (direction, step.and_then(|x| x.parse::<u8>().ok()))
            {
                Some(Motions { direction, step })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[test]
fn part1() {
    let a = parse_input(INPUT);

    let knot = Knot::default();

    let mut rope = Rope::new(knot, 1);

    let b = a
        .iter()
        .map(|motion| rope.tail_movement(motion))
        .flat_map(|elem| elem)
        .collect::<HashSet<_>>();

    println!("{}", b.len());
    //println!("{:?}", b);
    // 6256
}

#[test]
fn part2() {
    let a = parse_input(INPUT);

    let knot = Knot::default();

    let mut rope = Rope::new(knot, 9);

    let b = a
        .iter()
        .map(|motion| rope.tail_movement(motion))
        .flat_map(|elem| elem)
        .collect::<HashSet<_>>();

    println!("{}", b.len());
    //println!("{:?}", b);
}
