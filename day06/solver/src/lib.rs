use std::collections::HashSet;

use bindings::exports::aoc2024::day06::solver;

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
    fn solve_a(area: Vec<Vec<bool>>, position: (u32, u32)) -> u32 {
        let area = Area::from(area);
        let mut guard = Guard::new(position.into(), Direction::North);
        let mut visited_location = HashSet::new();

        while guard.is_in_area(&area) {
            visited_location.insert(guard.position);
            let forward = guard.forward();
            if area.is_tree(forward) {
                guard.direction = guard.direction.right();
            } else {
                guard.position = forward;
            }
        }

        visited_location.len() as u32
    }

    fn solve_b(area: Vec<Vec<bool>>, position: (u32, u32)) -> u32 {
        let mut valid_block_locations = 0;
        let area = Area::from(area);
        let guard = Guard::new(position.into(), Direction::North);

        for x in 0..area.width() {
            for y in 0..area.height() {
                let block = Position {
                    x: x as i32,
                    y: y as i32,
                };
                if guard.position == block {
                    continue;
                }
                if area.is_tree(block) {
                    continue;
                }

                if area.is_loop_if_blocked(block, guard) {
                    valid_block_locations += 1;
                } else {
                }
            }
        }
        valid_block_locations
    }
}

bindings::export!(Component with_types_in bindings);

struct Area(Vec<Vec<bool>>);

impl From<Vec<Vec<bool>>> for Area {
    fn from(input: Vec<Vec<bool>>) -> Self {
        Area(input)
    }
}

impl Area {
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn in_area(&self, position: Position) -> bool {
        !self.out_of_area(position)
    }

    pub fn out_of_area(&self, position: Position) -> bool {
        position.x < 0
            || position.y < 0
            || position.x as usize >= self.width()
            || position.y as usize >= self.height()
    }

    pub fn is_tree(&self, position: Position) -> bool {
        self.in_area(position) && self.0[position.y as usize][position.x as usize]
    }

    pub fn is_loop_if_blocked(&self, block: Position, mut guard: Guard) -> bool {
        let mut seen_guard_positions = HashSet::new();

        while guard.is_in_area(&self) {
            if !seen_guard_positions.insert(guard) {
                return true;
            }
            let forward = guard.forward();
            if forward == block || self.is_tree(forward) {
                guard.direction = guard.direction.right();
            } else {
                guard.position = forward;
            }
        }

        false
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl From<(u32, u32)> for Position {
    fn from((x, y): (u32, u32)) -> Self {
        Position {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ({},{})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => f.write_str("↑"),
            Direction::South => f.write_str("↓"),
            Direction::East => f.write_str("→"),
            Direction::West => f.write_str("←"),
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    pub fn new(position: Position, direction: Direction) -> Self {
        Guard {
            position,
            direction,
        }
    }

    pub fn is_in_area(&self, area: &Area) -> bool {
        area.in_area(self.position)
    }

    pub fn forward(&self) -> Position {
        match self.direction {
            Direction::North => Position {
                x: self.position.x,
                y: self.position.y - 1,
            },
            Direction::South => Position {
                x: self.position.x,
                y: self.position.y + 1,
            },
            Direction::East => Position {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Direction::West => Position {
                x: self.position.x - 1,
                y: self.position.y,
            },
        }
    }
}

impl std::fmt::Debug for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.direction, self.position)
    }
}
