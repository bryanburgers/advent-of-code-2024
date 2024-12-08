use std::collections::{HashMap, HashSet};

use bindings::exports::aoc2024::day08::solver;

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
    fn solve_a(input: solver::Input) -> u64 {
        let area = Area {
            width: input.area_width,
            height: input.area_height,
        };
        let mut antenas_by_frequency = HashMap::new();
        for antena in input.antenas {
            let position = Position::from(antena.position);
            antenas_by_frequency
                .entry(antena.frequency)
                .or_insert_with(Vec::new)
                .push(position);
        }

        let mut antinode_positions = HashSet::new();
        for (_, positions) in antenas_by_frequency {
            for (position_one, position_two) in pairs(&positions) {
                let (antinode_one, antinode_two) =
                    Position::antinodes(*position_one, *position_two);
                if area.contains_position(antinode_one) {
                    antinode_positions.insert(antinode_one);
                }
                if area.contains_position(antinode_two) {
                    antinode_positions.insert(antinode_two);
                }
            }
        }

        for y in 0..area.height {
            let mut debug = String::new();
            for j in 0..area.width {
                let position = Position { x: j, y };
                if antinode_positions.contains(&position) {
                    debug.push('#');
                } else {
                    debug.push('.');
                }
            }
            info!("{}", debug);
        }

        antinode_positions.len() as u64
    }

    fn solve_b(input: solver::Input) -> u64 {
        0
    }
}

bindings::export!(Component with_types_in bindings);

#[derive(Debug)]
struct Area {
    width: i32,
    height: i32,
}

impl Area {
    pub fn contains_position(&self, position: Position) -> bool {
        position.x >= 0 && position.x < self.width && position.y >= 0 && position.y < self.height
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl From<solver::Position> for Position {
    fn from(position: solver::Position) -> Self {
        Position {
            x: position.x,
            y: position.y,
        }
    }
}

impl Position {
    pub fn antinodes(one: Position, two: Position) -> (Position, Position) {
        let x_diff = two.x - one.x;
        let y_diff = two.y - one.y;
        let antinode_one = Position {
            x: one.x - x_diff,
            y: one.y - y_diff,
        };
        let antinode_two = Position {
            x: two.x + x_diff,
            y: two.y + y_diff,
        };
        (antinode_one, antinode_two)
    }
}

pub fn pairs<T>(slice: &[T]) -> PairIterator<T> {
    PairIterator { slice, i: 0, j: 1 }
}

pub struct PairIterator<'a, T> {
    slice: &'a [T],
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for PairIterator<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.slice.len() || self.j >= self.slice.len() {
            return None;
        }

        let pair = (
            self.slice.get(self.i).unwrap(),
            self.slice.get(self.j).unwrap(),
        );

        self.j += 1;
        if self.j >= self.slice.len() {
            self.i += 1;
            self.j = self.i + 1;
        }

        Some(pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antinodes() {
        let position_one = Position { x: 4, y: 3 };
        let position_two = Position { x: 5, y: 5 };
        let (antinode_one, antinode_two) = Position::antinodes(position_one, position_two);
        assert_eq!(antinode_one, Position { x: 3, y: 1 });
        assert_eq!(antinode_two, Position { x: 6, y: 7 });
    }
}
