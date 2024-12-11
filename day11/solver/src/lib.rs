use std::collections::{HashMap, VecDeque};

use bindings::exports::aoc2024::day11::solver;

#[allow(warnings)]
mod bindings;

struct Component;

#[allow(unused_macros)]
macro_rules! info {
    ($($arg:tt)*) => {
        bindings::aoc::base::debug::info(&format!($($arg)*));
    };
}

impl solver::Guest for Component {
    fn solve_a(input: Vec<u64>) -> u64 {
        let mut stones = input.into_iter().map(Stone::from).collect::<Vec<_>>();

        for _ in 0..25 {
            for stone in stones.iter_mut() {
                stone.step();
            }
        }

        stones.iter().map(|f| f.count()).sum()
    }

    fn solve_b(input: Vec<u64>) -> u64 {
        const STEPS: u64 = 75;
        let mut cache = HashMap::new();

        let mut count = 0;
        for i in input {
            count += recursive_with_cache(i, STEPS, &mut cache);
        }
        count
    }
}

bindings::export!(Component with_types_in bindings);

fn recursive_with_cache(value: u64, steps: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if steps == 0 {
        return 1;
    }

    if let Some(&count) = cache.get(&(value, steps)) {
        count
    } else {
        let count = match step(value) {
            Either::Left(value) => recursive_with_cache(value, steps - 1, cache),
            Either::Right((top, bottom)) => {
                recursive_with_cache(top, steps - 1, cache)
                    + recursive_with_cache(bottom, steps - 1, cache)
            }
        };
        cache.insert((value, steps), count);
        count
    }
}

enum Stone {
    Value(u64),
    Pair(Box<Stone>, Box<Stone>),
}

impl From<u64> for Stone {
    fn from(value: u64) -> Self {
        Stone::Value(value)
    }
}

impl Stone {
    fn step(&mut self) {
        match self {
            Stone::Value(value) => {
                *self = match step(*value) {
                    Either::Left(value) => Stone::Value(value),
                    Either::Right((top, bottom)) => {
                        Stone::Pair(Box::new(Stone::from(top)), Box::new(Stone::from(bottom)))
                    }
                };
            }
            Stone::Pair(left, right) => {
                left.step();
                right.step();
            }
        }
    }

    fn count(&self) -> u64 {
        match self {
            Stone::Value(_) => 1,
            Stone::Pair(left, right) => left.count() + right.count(),
        }
    }

    fn items(&self) -> StoneIterator {
        StoneIterator::new(self)
    }
}

impl std::fmt::Debug for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut is_first = true;
        for item in self.items() {
            if is_first {
                is_first = false;
            } else {
                f.write_str(", ")?;
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

struct StoneIterator<'a> {
    stack: VecDeque<&'a Stone>,
}

impl<'a> StoneIterator<'a> {
    fn new(root: &'a Stone) -> Self {
        let mut stack = VecDeque::new();
        stack.push_front(root);
        Self { stack }
    }
}

impl<'a> Iterator for StoneIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.stack.pop_front()?;

            match current {
                Stone::Value(value) => {
                    return Some(*value);
                }
                Stone::Pair(left, right) => {
                    self.stack.push_front(right);
                    self.stack.push_front(left);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn step(input: u64) -> Either<u64, (u64, u64)> {
    if input == 0 {
        Either::Left(1)
    } else if let Some((top, bottom)) = split(input) {
        Either::Right((top, bottom))
    } else {
        Either::Left(input * 2024)
    }
}

fn split(input: u64) -> Option<(u64, u64)> {
    if input < 10 {
        return None;
    }

    let mut low = 10;
    let mut high = 100;
    let mut split = 10;

    while high <= input {
        low *= 100;
        high *= 100;
        split *= 10;
    }

    if low <= input && input < high {
        let top = input / split;
        let bottom = input % split;
        Some((top, bottom))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        assert_eq!(step(0), Either::Left(1));
        assert_eq!(step(1), Either::Left(2024));
        assert_eq!(step(10), Either::Right((1, 0)));
        assert_eq!(step(99), Either::Right((9, 9)));
        assert_eq!(step(999), Either::Left(2021976));
    }

    #[test]
    fn test_split() {
        assert_eq!(split(10), Some((1, 0)));
        assert_eq!(split(80), Some((8, 0)));
        assert_eq!(split(87), Some((8, 7)));
        assert_eq!(split(100), None);
        assert_eq!(split(1000), Some((10, 0)));
        assert_eq!(split(1010), Some((10, 10)));
        assert_eq!(split(1234), Some((12, 34)));
        assert_eq!(split(12345), None);
        assert_eq!(split(123456), Some((123, 456)));
    }

    #[test]
    fn test_iterator() {
        let stone = Stone::Pair(
            Box::new(Stone::Pair(
                Box::new(Stone::from(1)),
                Box::new(Stone::from(2)),
            )),
            Box::new(Stone::from(3)),
        );

        let mut iterator = stone.items();
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), None);
    }
}
