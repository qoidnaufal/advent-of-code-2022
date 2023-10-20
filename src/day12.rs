use std::collections::{HashMap, VecDeque};

static INPUT: &'static str = include_str!("./day12.txt");

#[derive(Debug, Clone)]
struct HeightMap {
    data: Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl HeightMap {
    fn get_surrounding_index(&self, loc: (usize, usize)) -> Vec<(usize, usize)> {
        // down, up, left, right
        let direction = [(0, 1), (0, -1), (-1, 0), (1, 0)];

        direction
            .iter()
            .filter_map(|(x, y)| {
                let dx = loc.0 as i32 + x;
                let dy = loc.1 as i32 + y;

                if dx >= 0
                    && dy >= 0
                    && dy as usize <= self.data.len() - 1
                    && dx as usize <= self.data[loc.1].len() - 1
                {
                    Some((dx as usize, dy as usize))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn next_valid_destination(&self, loc: (usize, usize)) -> Vec<(usize, usize)> {
        let current_value = self.data[loc.1][loc.0];

        let surrounding_index = self.get_surrounding_index(loc);

        surrounding_index
            .iter()
            .filter_map(|&idx| {
                let dest_val = self.data[idx.1][idx.0];
                if current_value + 1 == dest_val || current_value >= dest_val {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn climb(&self, start: (usize, usize)) -> HashMap<(usize, usize), usize> {
        let mut to_visit = VecDeque::new();

        let mut path = HashMap::new();
        path.insert(start, 0usize);

        to_visit.extend(self.next_valid_destination(start));

        while let Some(valid_dest) = to_visit.pop_front() {
            let next_valid = self.next_valid_destination(valid_dest);

            let surrounding_index = self.get_surrounding_index(valid_dest);

            let next_path_dist = surrounding_index
                .iter()
                .filter_map(|pos| path.get(pos))
                .map(|&x| x + 1)
                .max();

            if next_path_dist.is_none() {
                continue;
            }

            let current_dist = path.entry(valid_dest).or_insert(usize::MAX);

            if *current_dist > next_path_dist.unwrap() {
                *current_dist = next_path_dist.unwrap();
                to_visit.extend(next_valid);
            }
        }
        path
    }
}

fn parse_input(s: &str) -> HeightMap {
    let input = s
        .lines()
        .map(|line| line.chars().map(|char| char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_y = input.iter().position(|vec| vec.contains(&'S')).unwrap();
    let start_x = input[start_y].iter().position(|chr| chr == &'S').unwrap();

    let start = (start_x, start_y);

    let end_y = input.iter().position(|vec| vec.contains(&'E')).unwrap();
    let end_x = input[end_y].iter().position(|chr| chr == &'E').unwrap();

    let end = (end_x, end_y);

    let data = input
        .iter()
        .map(|vec| {
            vec.iter()
                .map(|&chr| {
                    if chr.is_ascii_lowercase() {
                        chr as usize - 96
                    } else if chr.is_ascii_uppercase() {
                        let chr = match chr {
                            'S' => 'a',
                            'E' => 'z',
                            _ => unreachable!(),
                        };
                        chr as usize - 96
                    } else {
                        unreachable!()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    HeightMap { data, start, end }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let a = parse_input(INPUT);

        let b = a.climb(a.start);

        println!(
            "\n-----point is: {:?},\n-----distance is: {:?}",
            a.end,
            b.get(&a.end)
        );
    }

    #[test]
    fn part2() {
        let a = parse_input(INPUT);

        let mut coordinate_of_ones = vec![];

        for i in a.data.iter() {
            if i.contains(&1) {
                let idx_y = a.data.iter().position(|vec| vec == i).unwrap();

                for idx_x in 0..a.data[idx_y].len() {
                    if a.data[idx_y][idx_x] == 1 {
                        coordinate_of_ones.push((idx_x, idx_y));
                    }
                }
            }
        }

        let min_distance_from_pos_a = coordinate_of_ones
            .iter()
            .filter_map(|&start| a.climb(start).get(&a.end).cloned())
            .min();

        println!("{:?}", min_distance_from_pos_a);
    }
}
