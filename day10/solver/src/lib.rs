use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use bindings::exports::aoc2024::day10::solver;

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
    fn solve_a(input: solver::TopographicalMapBorrow<'_>) -> u64 {
        let map = Map::from(input);

        let mut accum = 0_u64;
        let mut reachability = HashMap::new();
        for y in 0..map.height() {
            for x in 0..map.width() {
                let point = Point {
                    x: x as i64,
                    y: y as i64,
                };
                if map.at(point) == 0 {
                    let r = map.reachable(point, &mut reachability);
                    accum += r.len() as u64;
                }
            }
        }

        accum
    }

    fn solve_b(input: solver::TopographicalMapBorrow<'_>) -> u64 {
        let map = Map::from(input);

        let mut accum = 0_u64;
        let mut known_ratings = HashMap::new();
        for y in 0..map.height() {
            for x in 0..map.width() {
                let point = Point {
                    x: x as i64,
                    y: y as i64,
                };
                if map.at(point) == 0 {
                    accum += map.rating(point, &mut known_ratings);
                }
            }
        }

        accum
    }
}

struct MyTopographicalMap(Vec<Vec<u8>>);

impl bindings::exports::aoc2024::day10::types::Guest for Component {
    type TopographicalMap = MyTopographicalMap;
}

impl bindings::exports::aoc2024::day10::types::GuestTopographicalMap for MyTopographicalMap {
    fn new(map: Vec<Vec<u8>>) -> Self {
        MyTopographicalMap(map)
    }

    fn map_width(&self) -> u32 {
        self.0[0].len() as u32
    }

    fn map_height(&self) -> u32 {
        self.0.len() as u32
    }

    fn height_at_location(&self, x: u32, y: u32) -> u8 {
        self.0[y as usize][x as usize]
    }
}

bindings::export!(Component with_types_in bindings);

struct Map<'a> {
    data: solver::TopographicalMapBorrow<'a>,
}

impl<'a> From<solver::TopographicalMapBorrow<'a>> for Map<'a> {
    fn from(data: solver::TopographicalMapBorrow<'a>) -> Self {
        Self { data }
    }
}

impl Map<'_> {
    pub fn width(&self) -> usize {
        use crate::bindings::exports::aoc2024::day10::types::GuestTopographicalMap;
        self.data.get::<MyTopographicalMap>().map_width() as usize
    }

    pub fn height(&self) -> usize {
        use crate::bindings::exports::aoc2024::day10::types::GuestTopographicalMap;
        self.data.get::<MyTopographicalMap>().map_height() as usize
    }

    pub fn at(&self, point: Point) -> u8 {
        assert!(self.in_bounds(point));
        use crate::bindings::exports::aoc2024::day10::types::GuestTopographicalMap;
        self.data
            .get::<MyTopographicalMap>()
            .height_at_location(point.x as u32, point.y as u32)
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x < self.width() as i64
            && point.y < self.height() as i64
    }

    pub fn reachable(
        &self,
        point: Point,
        known_paths: &mut HashMap<Point, HashSet<Point>>,
    ) -> HashSet<Point> {
        if !self.in_bounds(point) {
            return HashSet::new();
        }

        if let Some(r) = known_paths.get(&point) {
            return r.clone();
        }

        let value = self.at(point);
        if value == 9 {
            return HashSet::from([point]);
        }

        let mut accum = HashSet::new();
        for pt in [point.north(), point.south(), point.east(), point.west()]
            .into_iter()
            .filter(|pt| self.in_bounds(*pt))
        {
            let neighbor = self.at(pt);
            if neighbor == value + 1 {
                let r = self.reachable(pt, known_paths);
                accum = accum.union(&r).copied().collect::<HashSet<_>>();
            }
        }

        known_paths.insert(point, accum.clone());

        accum
    }

    pub fn rating(&self, point: Point, known_ratings: &mut HashMap<Point, u64>) -> u64 {
        if !self.in_bounds(point) {
            return 0;
        }

        if let Some(r) = known_ratings.get(&point) {
            return *r;
        }

        let value = self.at(point);
        if value == 9 {
            return 1;
        }

        let mut accum = 0;
        for pt in [point.north(), point.south(), point.east(), point.west()]
            .into_iter()
            .filter(|pt| self.in_bounds(*pt))
        {
            let neighbor = self.at(pt);
            if neighbor == value + 1 {
                accum += self.rating(pt, known_ratings);
            }
        }

        known_ratings.insert(point, accum);

        accum
    }
}

impl Debug for Map<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.width() {
            for x in 0..self.height() {
                let point = Point {
                    x: x as i64,
                    y: y as i64,
                };
                write!(f, "{}", self.at(point))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn north(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn south(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn east(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn west(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
