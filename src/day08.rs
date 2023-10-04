use std::{collections::HashMap, convert::Infallible, str::FromStr};

static INPUT: &'static str = include_str!("./day08.txt");

type Position = (usize, usize);

#[derive(Debug)]
struct Forest {
    trees: HashMap<Position, u8>,
}

impl FromStr for Forest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Forest {
            trees: s
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, chr)| {
                        chr.to_string()
                            .parse::<u8>()
                            .ok()
                            .map(|height| ((x, y), height))
                    })
                })
                .collect::<HashMap<_, _>>(),
        })
    }
}

impl Forest {
    fn is_visible(&self, (x, y): Position) -> bool {
        let height = self.trees.get(&(x, y)).copied().unwrap_or(0);

        fn check_height(forest: &Forest, iter: impl Iterator<Item = Position>, height: u8) -> bool {
            iter.filter_map(|pos| forest.trees.get(&pos))
                .all(|h| *h < height)
        }

        check_height(self, (0..x).map(|x| (x, y)), height)
            || check_height(self, (0..y).map(|y| (x, y)), height)
            || check_height(
                self,
                (x + 1..)
                    .map(|x| (x, y))
                    .take_while(|pos| self.trees.contains_key(pos)),
                height,
            )
            || check_height(
                self,
                (y + 1..)
                    .map(|y| (x, y))
                    .take_while(|pos| self.trees.contains_key(pos)),
                height,
            )
    }

    fn scenic_score(&self, (x, y): Position) -> usize {
        let height = self.trees.get(&(x, y)).copied().unwrap_or(0);

        let count_trees = |iter: &mut dyn Iterator<Item = Position>| -> usize {
            let heights = iter
                .filter_map(|pos| self.trees.get(&pos).copied())
                .collect::<Vec<_>>();

            let mut result = heights.iter().take_while(|h| **h < height).count();

            if let Some(final_height) = heights.get(result) {
                if final_height >= &height {
                    result += 1
                }
            }

            result
        };

        let left_trees_count = count_trees(&mut (0..x).rev().map(|x| (x, y)));
        let top_trees_count = count_trees(&mut (0..y).rev().map(|y| (x, y)));
        let right_trees_count = count_trees(
            &mut (x + 1..)
                .map(|x| (x, y))
                .take_while(|pos| self.trees.contains_key(pos)),
        );
        let bottom_trees_count = count_trees(
            &mut (y + 1..)
                .map(|y| (x, y))
                .take_while(|pos| self.trees.contains_key(pos)),
        );

        left_trees_count * top_trees_count * right_trees_count * bottom_trees_count
    }
}

#[test]
fn part1() {
    let forest = INPUT.parse::<Forest>().unwrap();

    let result = forest
        .trees
        .keys()
        .filter(|pos| forest.is_visible(**pos))
        .count();

    println!("{:?}", result);
}

#[test]
fn part2() {
    let forest = INPUT.parse::<Forest>().unwrap();

    let result = forest
        .trees
        .keys()
        .map(|pos| forest.scenic_score(*pos))
        .max()
        .unwrap();

    println!("{}", result);
}
