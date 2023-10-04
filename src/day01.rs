#[allow(dead_code)]
static INPUT: &'static str = include_str!("./day01.txt");

#[derive(Debug)]
struct Elf {
    #[allow(dead_code)]
    food_items: Vec<u64>,
}

impl Elf {
    #[allow(dead_code)]
    fn sum_of_calories(&self) -> u64 {
        self.food_items.iter().sum()
    }
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<Elf> {
    let a = input
        .split("\n\n")
        .map(|input_elements| Elf {
            food_items: input_elements
                .lines()
                .filter_map(|calories| calories.parse::<u64>().ok())
                .collect(),
        })
        .collect();
    a
}

#[test]
fn part1() {
    let elves = parse_input(INPUT);

    let b = elves.iter().map(|elf| elf.sum_of_calories()).max().unwrap();

    println!("{:?}", b);
}

#[test]
fn part2() {
    let mut elves = parse_input(INPUT);
    elves.sort_by_cached_key(|elf| elf.sum_of_calories());
    elves.reverse();

    let b = elves
        .iter()
        .take(3)
        .map(|elf| elf.sum_of_calories())
        .sum::<u64>();

    println!("{:?}", b);
}
