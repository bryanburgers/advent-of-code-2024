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

        for region in map.regions() {
            cost += region.area() * region.perimeter();
        }

        cost
    }

    fn solve_b(input: Vec<Vec<char>>) -> u64 {
        let map = Map::from(input);

        let mut cost = 0;
        for region in map.regions() {
            cost += region.area() * region.sides();
        }

        cost
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
    fn regions(&self) -> RegionIterator {
        RegionIterator::new(self)
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

struct Region {
    points: HashSet<Point>,
}

impl Region {
    #[allow(dead_code)]
    pub fn representative(&self) -> Point {
        *self.points.iter().next().unwrap()
    }

    pub fn area(&self) -> u64 {
        self.points.len() as u64
    }

    pub fn perimeter(&self) -> u64 {
        let mut perimeter = 0;
        for point in &self.points {
            for neighbor_point in [point.north(), point.south(), point.west(), point.east()] {
                if !self.points.contains(&neighbor_point) {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    pub fn max_x(&self) -> i64 {
        self.points.iter().map(|point| point.x).max().unwrap()
    }
    pub fn min_x(&self) -> i64 {
        self.points.iter().map(|point| point.x).min().unwrap()
    }
    pub fn max_y(&self) -> i64 {
        self.points.iter().map(|point| point.y).max().unwrap()
    }
    pub fn min_y(&self) -> i64 {
        self.points.iter().map(|point| point.y).min().unwrap()
    }

    pub fn sides(&self) -> u64 {
        let mut sides = 0;

        let min_x = self.min_x();
        let min_y = self.min_y();
        let max_x = self.max_x();
        let max_y = self.max_y();

        {
            let mut sides_from_west = 0;
            let mut seen_from_west = HashSet::new();
            for y in min_y..=max_y {
                for x in min_x - 1..=max_x + 1 {
                    let point = Point { x, y };
                    if seen_from_west.contains(&point) {
                        continue;
                    }

                    let next_point = point.east();
                    if !self.contains(point) && self.contains(next_point) {
                        sides_from_west += 1;
                        seen_from_west.insert(point);

                        for new_y in y.. {
                            let new_point = Point { x, y: new_y };
                            let new_next_point = new_point.east();
                            if !self.contains(new_point) && self.contains(new_next_point) {
                                seen_from_west.insert(new_point);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            sides += sides_from_west;
        }

        {
            let mut sides_from_east = 0;
            let mut seen_from_east = HashSet::new();
            for y in min_y..=max_y {
                for x in (min_x - 1..=max_x + 1).rev() {
                    let point = Point { x, y };
                    if seen_from_east.contains(&point) {
                        continue;
                    }

                    let next_point = point.west();
                    if !self.contains(point) && self.contains(next_point) {
                        sides_from_east += 1;
                        seen_from_east.insert(point);

                        for new_y in y.. {
                            let new_point = Point { x, y: new_y };
                            let new_next_point = new_point.west();
                            if !self.contains(new_point) && self.contains(new_next_point) {
                                seen_from_east.insert(new_point);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            sides += sides_from_east;
        }

        {
            let mut sides_from_north = 0;
            let mut seen_from_north = HashSet::new();
            for x in min_x..=max_x {
                for y in min_y - 1..=max_y + 1 {
                    let point = Point { x, y };
                    if seen_from_north.contains(&point) {
                        continue;
                    }

                    let next_point = point.south();
                    if !self.contains(point) && self.contains(next_point) {
                        sides_from_north += 1;
                        seen_from_north.insert(point);

                        for new_x in x.. {
                            let new_point = Point { x: new_x, y };
                            let new_next_point = new_point.south();
                            if !self.contains(new_point) && self.contains(new_next_point) {
                                seen_from_north.insert(new_point);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            sides += sides_from_north;
        }

        {
            let mut sides_from_south = 0;
            let mut seen_from_south = HashSet::new();
            for x in min_x..=max_x {
                for y in min_y - 1..=max_y + 1 {
                    let point = Point { x, y };
                    if seen_from_south.contains(&point) {
                        continue;
                    }

                    let next_point = point.north();
                    if !self.contains(point) && self.contains(next_point) {
                        sides_from_south += 1;
                        seen_from_south.insert(point);

                        for new_x in x.. {
                            let new_point = Point { x: new_x, y };
                            let new_next_point = new_point.north();
                            if !self.contains(new_point) && self.contains(new_next_point) {
                                seen_from_south.insert(new_point);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            sides += sides_from_south;
        }

        sides
    }

    pub fn contains(&self, point: Point) -> bool {
        self.points.contains(&point)
    }
}

struct RegionIterator<'a> {
    map: &'a Map,
    seen: HashSet<Point>,
    x: i64,
    y: i64,
}

impl<'a> RegionIterator<'a> {
    fn new(map: &'a Map) -> Self {
        Self {
            map,
            seen: HashSet::new(),
            x: 0,
            y: 0,
        }
    }

    fn region_at_point(&mut self, point: Point) -> Option<Region> {
        let mut stack = Vec::new();
        stack.push(point);

        let mut region_points = HashSet::new();
        while let Some(point) = stack.pop() {
            if self.seen.contains(&point) {
                continue;
            }

            self.seen.insert(point);
            region_points.insert(point);

            let value = self.map.get(point);

            for neighbor_point in [point.north(), point.south(), point.west(), point.east()] {
                let value_at_neighbor = self.map.get(neighbor_point);
                if value_at_neighbor == value {
                    stack.push(neighbor_point);
                }
            }
        }

        if region_points.is_empty() {
            None
        } else {
            Some(Region {
                points: region_points,
            })
        }
    }
}

impl Iterator for RegionIterator<'_> {
    type Item = Region;

    fn next(&mut self) -> Option<Self::Item> {
        while self.y < self.map.height() {
            while self.x < self.map.width() {
                let point = Point {
                    x: self.x,
                    y: self.y,
                };
                self.x += 1;

                if let Some(region) = self.region_at_point(point) {
                    return Some(region);
                }
            }

            self.x = 0;
            self.y += 1;
        }

        None
    }
}
