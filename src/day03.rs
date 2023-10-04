// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.

use std::collections::HashSet;

#[allow(dead_code)]
static INPUT: &'static str = include_str!("./day03.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Element(char);

impl Element {
    #[allow(dead_code)]
    fn from_char(char: char) -> Result<Self, char> {
        if char.is_ascii_alphabetic() {
            Ok(Self(char))
        } else {
            Err(char)
        }
    }

    #[allow(dead_code)]
    fn priority(&self) -> u64 {
        if self.0.is_ascii_lowercase() {
            self.0 as u64 - 97 + 1
        } else if self.0.is_ascii_uppercase() {
            self.0 as u64 - 65 + 27
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct RuckSack {
    #[allow(dead_code)]
    compartment_1: Vec<Element>,
    #[allow(dead_code)]
    compartment_2: Vec<Element>
}

impl RuckSack {
    #[allow(dead_code)]
    fn compare(&self) -> HashSet<Element> {
        self.compartment_1.iter()
            .collect::<HashSet<_>>()
            .intersection(&self.compartment_2
                .iter()
                .collect::<HashSet<_>>())
            .cloned()
            .cloned()
            .collect::<HashSet<_>>()
    }

    #[allow(dead_code)]
    fn itering(&self) -> impl Iterator<Item = &Element> + '_ {
        self.compartment_1.iter()
            .chain(self.compartment_2.iter())
    }
}

#[allow(dead_code)]
fn parse_input(input: &str) -> Vec<RuckSack> {
    input
        .lines()
        .map(|elems| {
            let slice_of_char = elems
                .chars()
                .filter_map(|chr| Element::from_char(chr).ok())
                .collect::<Vec<_>>();
            
            let compartmet_size = slice_of_char.len() / 2;

            RuckSack {
                compartment_1: slice_of_char[..compartmet_size].to_vec(),
                compartment_2: slice_of_char[compartmet_size..].to_vec()
            }
        }).collect()
}

#[test]
fn part1() {
    let a = parse_input(INPUT);

    let b = a.iter()
        .map(|rsck| rsck
            .compare()
            .iter()
            .map(|elm| elm
                .priority())
            .sum::<u64>())
        .sum::<u64>();

    println!("{:?}", b);
}

#[test]
fn part2() {
    let a = parse_input(INPUT);

    let b = (0..a.len())
        .step_by(3)
        .map(|idx| &a[idx..idx + 3])
        .filter_map(|rsck| {
            rsck.iter()
                .fold(None::<HashSet<_>>, |acc, abc| {
                    acc.map_or_else(
                        || Some(abc.itering().collect()),
                        |acc| {
                            Some(acc.intersection(&abc.itering().collect())
                            .copied()
                            .collect())
                        }
                    )
                })
        })
        .flat_map(|intersection| intersection.into_iter())
        .map(|item| item.priority())
        .sum::<u64>();

    println!("{:?}", b);
}