use std::{
    cmp::Ordering,
    convert::Infallible,
    str::{Chars, FromStr},
};

static INPUT: &'static str = include_str!("./day13.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Num(i32),
    List(Vec<Self>),
}

impl Packet {
    /// i had to look up to the code by UncleScientist
    /// because i don't quite get it on recursive parsing with FromStr
    fn parse_into(input: &mut Chars) -> Self {
        let mut result = Vec::new();
        let mut num = -1i32;

        while let Some(chr) = input.next() {
            match chr {
                '[' => result.push(Self::parse_into(input)),
                ',' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                        num = -1;
                    }
                }
                ']' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                    }
                    return Self::List(result);
                }
                '0'..='9' => {
                    if num == -1 {
                        num = (chr as u8 - b'0') as i32;
                    } else {
                        num = (num * 10) + (chr as u8 - b'0') as i32;
                    }
                }
                _ => unreachable!(),
            }
        }
        Self::List(result)
    }

    fn compare_packet(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(left), Self::List(right)) => {
                let mut idx = 0;
                while idx < left.len() && idx < right.len() {
                    match (&left[idx], &right[idx]) {
                        (Self::Num(l), Self::Num(r)) => {
                            if l != r {
                                return l.cmp(r);
                            }
                        }
                        (Self::List(_), Self::Num(r)) => {
                            let check = left[idx].compare_packet(&Self::List(vec![Self::Num(*r)]));
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Self::Num(l), Self::List(_)) => {
                            let check = Self::List(vec![Self::Num(*l)]).compare_packet(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Self::List(_), Self::List(_)) => {
                            let check = left[idx].compare_packet(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                    }
                    idx += 1;
                }
                left.len().cmp(&right.len())
            }
            _ => unreachable!(),
        }
    }
}

impl FromStr for Packet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(Self::parse_into(&mut chars))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare_packet(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Signal {
    left_packet: Packet,
    right_packet: Packet,
}

impl IntoIterator for Signal {
    type Item = Packet;
    type IntoIter = SignalIterator;

    fn into_iter(self) -> Self::IntoIter {
        SignalIterator {
            signal: self,
            index: 0,
        }
    }
}

struct SignalIterator {
    signal: Signal,
    index: usize,
}

impl Iterator for SignalIterator {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.signal.left_packet.clone(),
            1 => self.signal.right_packet.clone(),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

fn parse_input(s: &str) -> Vec<Signal> {
    s.split("\n\n")
        .map(|blocks| {
            let mut line = blocks.lines();

            let a = line.next().unwrap();
            let b = line.next().unwrap();

            Signal {
                left_packet: a.parse::<Packet>().unwrap(),
                right_packet: b.parse::<Packet>().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let a = parse_input(INPUT);

        let b = a
            .iter()
            .enumerate()
            .map(|(idx, signal)| {
                let mut marker = 0;
                if signal.left_packet < signal.right_packet {
                    marker += idx + 1;
                }
                marker
            })
            .sum::<usize>();

        println!("{:?}", b);
    }

    #[test]
    fn part2() {
        let a = parse_input(INPUT);

        let two = Packet::from_str("[[2]]").unwrap();
        let six = Packet::from_str("[[6]]").unwrap();

        let mut b = a
            .iter()
            .map(|signal| signal.clone().into_iter())
            .flat_map(|s| s)
            .collect::<Vec<_>>();

        b.push(two.clone());
        b.push(six.clone());

        b.sort();

        let position_of_two = b.iter().position(|packet| *packet == two).unwrap() + 1;
        let position_of_six = b.iter().position(|packet| *packet == six).unwrap() + 1;

        println!("{:?}", position_of_two * position_of_six);
    }
}
