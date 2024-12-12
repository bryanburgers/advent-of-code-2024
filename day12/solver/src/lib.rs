use std::collections::HashSet;

use bindings::exports::aoc2024::day12::solver;

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
    fn solve_a(input: Vec<Vec<char>>) -> u64 {
        let map = Map::from(input);
        let mut cost = 0;

        let mut solver = Solver::default();
        for y in 0..map.height() {
            for x in 0..map.width() {
                let point = Point { x, y };

                if let Some((area, perimeter)) = solver.solve_at_point(point, &map) {
                    cost += area * perimeter
                }
            }
        }

        cost
    }

    fn solve_b(_input: Vec<Vec<char>>) -> u64 {
        0
    }
}

bindings::export!(Component with_types_in bindings);

struct Map(Vec<Vec<char>>);

impl From<Vec<Vec<char>>> for Map {
    fn from(input: Vec<Vec<char>>) -> Self {
        Map(input)
    }
}

impl Map {
    fn width(&self) -> i64 {
        self.0[0].len() as i64
    }
    fn height(&self) -> i64 {
        self.0.len() as i64
    }
    fn get(&self, point: Point) -> Option<char> {
        self.0
            .get(point.y as usize)
            .and_then(|row| row.get(point.x as usize).copied())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn north(self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn south(self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn west(self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn east(self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Default)]
struct Solver {
    seen: HashSet<Point>,
}

impl Solver {
    fn solve_at_point(&mut self, point: Point, map: &Map) -> Option<(u64, u64)> {
        let mut stack = Vec::new();
        stack.push(point);

        let mut area = 0;
        let mut perimeter = 0;
        while let Some(point) = stack.pop() {
            if self.seen.contains(&point) {
                continue;
            }

            area += 1;
            self.seen.insert(point);

            let value = map.get(point);

            for neighbor_point in [point.north(), point.south(), point.west(), point.east()] {
                let value_at_neighbor = map.get(neighbor_point);
                if value_at_neighbor == value {
                    stack.push(neighbor_point);
                } else {
                    perimeter += 1;
                }
            }
        }

        if area == 0 {
            None
        } else {
            Some((area, perimeter))
        }
    }
}
