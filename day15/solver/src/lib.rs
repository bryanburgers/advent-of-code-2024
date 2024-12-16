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

    fn solve_b(input: solver::Input) -> i64 {
        let map = Map::from(&input);
        let mut map = MapWide::from(map);
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

    #[allow(dead_code)]
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

    fn widen(self) -> Position {
        Position {
            x: self.x * 2,
            y: self.y,
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

struct MapWide {
    walls: HashSet<Position>,
    boxes: HashSet<Position>,
    lantern_fish: Position,
}

impl From<Map> for MapWide {
    fn from(map: Map) -> Self {
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let lantern_fish = map.lantern_fish.widen();

        for tile in map.tiles {
            match tile.1 {
                TileType::Wall => {
                    walls.insert(tile.0.widen());
                    walls.insert(tile.0.widen().move_in_direction(Move::East));
                }
                TileType::Box => {
                    boxes.insert(tile.0.widen());
                }
            }
        }

        Self {
            walls,
            boxes,
            lantern_fish,
        }
    }
}

impl MapWide {
    pub fn min_x(&self) -> i64 {
        self.walls.iter().map(|position| position.x).min().unwrap()
    }
    pub fn min_y(&self) -> i64 {
        self.walls.iter().map(|position| position.y).min().unwrap()
    }
    pub fn max_x(&self) -> i64 {
        self.walls.iter().map(|position| position.x).max().unwrap()
    }
    pub fn max_y(&self) -> i64 {
        self.walls.iter().map(|position| position.y).max().unwrap()
    }

    /// Determine if there is a box at the given position. If there is, return the left coordinate
    /// for the box.
    pub fn box_at_position(&self, position: Position) -> Option<BoxPosition> {
        if self.boxes.contains(&position) {
            return Some(BoxPosition {
                left: position,
                right: position.move_in_direction(Move::East),
            });
        }
        if self.boxes.contains(&position.move_in_direction(Move::West)) {
            return Some(BoxPosition {
                left: position.move_in_direction(Move::West),
                right: position,
            });
        }
        None
    }

    pub fn boxes_in_direction_of_box(&self, position: BoxPosition, direction: Move) -> Boxes {
        match direction {
            Move::North | Move::South => {
                let box_at_position_one =
                    self.box_at_position(position.left.move_in_direction(direction));
                let box_at_position_two =
                    self.box_at_position(position.right.move_in_direction(direction));
                match (box_at_position_one, box_at_position_two) {
                    (None, None) => Boxes::Zero,
                    (Some(b), None) => Boxes::One(b),
                    (None, Some(b)) => Boxes::One(b),
                    (Some(b1), Some(b2)) if b1 == b2 => Boxes::One(b1),
                    (Some(b1), Some(b2)) => Boxes::Two(b1, b2),
                }
            }
            Move::East => self
                .box_at_position(position.right.move_in_direction(direction))
                .into(),

            Move::West => self
                .box_at_position(position.left.move_in_direction(direction))
                .into(),
        }
    }

    pub fn can_box_move(&self, position: BoxPosition, direction: Move) -> bool {
        assert!(self.boxes.contains(&position.left));

        let in_direction_left = position.left.move_in_direction(direction);
        let in_direction_right = position.right.move_in_direction(direction);

        if self.walls.contains(&in_direction_left) {
            return false;
        }
        if self.walls.contains(&in_direction_right) {
            return false;
        }

        match self.boxes_in_direction_of_box(position, direction) {
            Boxes::Zero => true,
            Boxes::One(box_position) => self.can_box_move(box_position, direction),
            Boxes::Two(p1, p2) => {
                self.can_box_move(p1, direction) && self.can_box_move(p2, direction)
            }
        }
    }

    pub fn can_lanternfish_move(&self, direction: Move) -> bool {
        let in_direction = self.lantern_fish.move_in_direction(direction);
        if self.walls.contains(&in_direction) {
            return false;
        }

        if let Some(box_position) = self.box_at_position(in_direction) {
            return self.can_box_move(box_position, direction);
        }

        true
    }

    // Precondition: can_lanternfish_move must have returned true
    fn perform_lanternfish_move(&mut self, direction: Move) {
        let in_direction = self.lantern_fish.move_in_direction(direction);
        if self.walls.contains(&in_direction) {
            panic!("Lanternfish cannot move in this direction");
        }

        if let Some(box_position) = self.box_at_position(in_direction) {
            self.perform_box_move(box_position, direction);
        }

        self.lantern_fish = in_direction;
    }

    // Precondition: can_box_move must have returned true
    fn perform_box_move(&mut self, position: BoxPosition, direction: Move) {
        assert!(self.boxes.contains(&position.left));

        let in_direction_left = position.left.move_in_direction(direction);
        let in_direction_right = position.right.move_in_direction(direction);

        if self.walls.contains(&in_direction_left) {
            panic!("Box cannot move in this direction");
        }
        if self.walls.contains(&in_direction_right) {
            panic!("Box cannot move in this direction");
        }

        match self.boxes_in_direction_of_box(position, direction) {
            Boxes::Zero => {}
            Boxes::One(box_position) => self.perform_box_move(box_position, direction),
            Boxes::Two(p1, p2) => {
                self.perform_box_move(p1, direction);
                self.perform_box_move(p2, direction);
            }
        }

        self.boxes.remove(&position.left);
        self.boxes.insert(in_direction_left);
    }

    pub fn move_once(&mut self, direction: Move) {
        if self.can_lanternfish_move(direction) {
            self.perform_lanternfish_move(direction);
        }
    }

    pub fn gps(&self) -> i64 {
        self.boxes.iter().map(|position| position.gps()).sum()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct BoxPosition {
    left: Position,
    right: Position,
}

enum Boxes {
    Zero,
    One(BoxPosition),
    Two(BoxPosition, BoxPosition),
}

impl From<Option<BoxPosition>> for Boxes {
    fn from(option: Option<BoxPosition>) -> Self {
        match option {
            None => Self::Zero,
            Some(b) => Self::One(b),
        }
    }
}

impl std::fmt::Display for MapWide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.min_x();
        let min_y = self.min_y();
        let max_x = self.max_x();
        let max_y = self.max_y();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let position = Position { x, y };
                let previous_position = position.move_in_direction(Move::West);
                if position == self.lantern_fish {
                    write!(f, "@")?;
                } else if self.boxes.contains(&position) {
                    write!(f, "[")?;
                } else if self.boxes.contains(&previous_position) {
                    write!(f, "]")?;
                } else if self.walls.contains(&position) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
