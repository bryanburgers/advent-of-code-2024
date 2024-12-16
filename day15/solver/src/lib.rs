use std::collections::{HashMap, HashSet};

use bindings::exports::aoc2024::day15::solver::{self};

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
    fn solve_a(input: solver::Input) -> i64 {
        let mut map = Map::from(&input);
        let moves = input
            .moves
            .into_iter()
            .map(|m| Move::from(m))
            .collect::<Vec<_>>();

        for m in moves {
            map.move_once(m);
        }

        map.gps()
    }

    fn solve_b(_input: solver::Input) -> i64 {
        0
    }
}

bindings::export!(Component with_types_in bindings);

struct Map {
    tiles: HashMap<Position, TileType>,
    lantern_fish: Position,
}

impl From<&solver::Input> for Map {
    fn from(input: &solver::Input) -> Self {
        let mut tiles = HashMap::new();
        let lantern_fish = Position::from(input.lantern_fish);

        for tile in &input.tiles {
            tiles.insert(Position::from(tile.position), TileType::from(tile.type_));
        }

        Self {
            tiles,
            lantern_fish,
        }
    }
}

impl Map {
    pub fn min_x(&self) -> i64 {
        self.tiles.keys().map(|position| position.x).min().unwrap()
    }
    pub fn min_y(&self) -> i64 {
        self.tiles.keys().map(|position| position.y).min().unwrap()
    }
    pub fn max_x(&self) -> i64 {
        self.tiles.keys().map(|position| position.x).max().unwrap()
    }
    pub fn max_y(&self) -> i64 {
        self.tiles.keys().map(|position| position.y).max().unwrap()
    }

    pub fn is_box(&self, position: Position) -> bool {
        matches!(self.tiles.get(&position), Some(TileType::Box))
    }

    pub fn is_wall(&self, position: Position) -> bool {
        matches!(self.tiles.get(&position), Some(TileType::Wall))
    }

    pub fn is_free(&self, position: Position) -> bool {
        self.tiles.get(&position).is_none()
    }

    pub fn first_free_position_in_direction(&self, direction: Move) -> Option<Position> {
        let mut position = self.lantern_fish.move_in_direction(direction);
        loop {
            if self.is_free(position) {
                return Some(position);
            }
            if self.is_wall(position) {
                return None;
            }
            position = position.move_in_direction(direction);
        }
    }

    pub fn move_once(&mut self, direction: Move) {
        let in_direction = self.lantern_fish.move_in_direction(direction);
        if self.is_free(in_direction) {
            self.lantern_fish = in_direction;
        } else if let Some(first_free_position) = self.first_free_position_in_direction(direction) {
            self.tiles.insert(first_free_position, TileType::Box);
            self.tiles.remove(&in_direction);
            self.lantern_fish = in_direction;
        }
    }

    pub fn gps(&self) -> i64 {
        self.tiles
            .iter()
            .filter(|(_, tile)| matches!(tile, TileType::Box))
            .map(|(position, _)| position.gps())
            .sum()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.min_x();
        let min_y = self.min_y();
        let max_x = self.max_x();
        let max_y = self.max_y();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let position = Position { x, y };
                if position == self.lantern_fish {
                    write!(f, "@")?;
                } else {
                    match self.tiles.get(&position) {
                        Some(TileType::Wall) => write!(f, "#")?,
                        Some(TileType::Box) => write!(f, "O")?,
                        None => write!(f, ".")?,
                    }
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl From<solver::Position> for Position {
    fn from(position: solver::Position) -> Self {
        Self {
            x: position.x,
            y: position.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    North,
    South,
    East,
    West,
}

impl From<solver::Move> for Move {
    fn from(m: solver::Move) -> Self {
        match m {
            solver::Move::North => Self::North,
            solver::Move::South => Self::South,
            solver::Move::East => Self::East,
            solver::Move::West => Self::West,
        }
    }
}

impl Position {
    fn move_in_direction(self, direction: Move) -> Self {
        match direction {
            Move::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Move::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Move::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Move::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn gps(self) -> i64 {
        self.y * 100 + self.x
    }
}

enum TileType {
    Wall,
    Box,
}

impl From<solver::TileType> for TileType {
    fn from(tile_type: solver::TileType) -> Self {
        match tile_type {
            solver::TileType::Wall => Self::Wall,
            solver::TileType::Box => Self::Box,
        }
    }
}
