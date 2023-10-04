#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
static INPUT: &'static str = include_str!("./day06.txt");

struct WindowIterator<I: Iterator> {
    n: usize,
    iter: I,
    buffer: VecDeque<I::Item>,
}

impl<I: Iterator> WindowIterator<I> {
    #[allow(dead_code)]
    fn new(n: usize, iter: I) -> Self {
        Self {
            n,
            iter,
            buffer: VecDeque::with_capacity(n),
        }
    }
}

impl<I> Iterator for WindowIterator<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = VecDeque<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.len() < self.n {
            self.buffer
                .extend((&mut self.iter).take(self.n - self.buffer.len()));

            Some(self.buffer.clone())
        } else if let Some(item) = self.iter.next() {
            self.buffer.pop_front();
            self.buffer.push_back(item);

            Some(self.buffer.clone())
        } else {
            None
        }
    }
}

#[test]
fn part1() {
    let chars = INPUT.chars();

    let mut window = WindowIterator::new(4, chars);

    let result = window
        .position(|elem| elem.into_iter().collect::<HashSet<_>>().len() == 4)
        .map(|i| i + 4);

    println!("{}", result.unwrap());
}

#[test]
fn part2() {
    let chars = INPUT.chars();

    let mut window = WindowIterator::new(14, chars);

    let result = window
        .position(|elem| elem.into_iter().collect::<HashSet<_>>().len() == 14)
        .map(|i| i + 14);

    println!("{}", result.unwrap());
}
