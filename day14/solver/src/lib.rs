use std::collections::{HashMap, HashSet};

use bindings::exports::aoc2024::day14::solver::{self};

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
        let input = Input::from(input);
        let mut quadrants = HashMap::new();
        let mut original_positions = Debugger::new(input.size);
        let mut final_positions = Debugger::new(input.size);
        for robot in &input.robots {
            original_positions.add(*robot);
            let stepped = robot.step_n(100, input.size);
            let quadrant = input.size.quadrant(stepped.position);
            *quadrants.entry(quadrant).or_insert(0) += 1;
            final_positions.add(stepped);
        }

        quadrants
            .into_iter()
            .filter(|(quad, _)| !matches!(quad, Quadrant::Liminal))
            .map(|(_, count)| count as u64)
            .product()
    }

    fn solve_b(input: solver::Input) -> u64 {
        let input = Input::from(input);
        let mut current_max = (0, 0);
        for i in 0..=10000 {
            let mut tree_detector = TreeDetector::default();
            let mut debugger = Debugger::new(input.size);
            for robot in &input.robots {
                let stepped = robot.step_n(i, input.size);
                debugger.add(stepped);
                tree_detector.add(stepped);
            }

            let score = tree_detector.score();
            if score > current_max.1 {
                current_max = (i, score);
                info!("After {i} steps, score={score}:\n{debugger}");
                info!("");
            }
        }

        current_max.0 as u64
    }
}

bindings::export!(Component with_types_in bindings);

struct Input {
    robots: Vec<Robot>,
    size: Size,
}

impl From<solver::Input> for Input {
    fn from(value: solver::Input) -> Self {
        Input {
            robots: value.robots.into_iter().map(Robot::from).collect(),
            size: Size::from(value.size),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl From<solver::Robot> for Robot {
    fn from(value: solver::Robot) -> Self {
        Robot {
            position: Position::from(value.position),
            velocity: Velocity::from(value.velocity),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Size {
    width: i64,
    height: i64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Liminal,
}

impl Size {
    fn middle_x(&self) -> i64 {
        self.width / 2
    }
    fn middle_y(&self) -> i64 {
        self.height / 2
    }
    fn quadrant(&self, position: Position) -> Quadrant {
        let position = position % *self;
        if position.x == self.middle_x() || position.y == self.middle_y() {
            return Quadrant::Liminal;
        }
        match (position.x < self.middle_x(), position.y < self.middle_y()) {
            (true, true) => Quadrant::TopLeft,
            (false, true) => Quadrant::TopRight,
            (true, false) => Quadrant::BottomLeft,
            (false, false) => Quadrant::BottomRight,
        }
    }
}

impl From<solver::Size> for Size {
    fn from(value: solver::Size) -> Self {
        Size {
            width: value.width,
            height: value.height,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl From<solver::Position> for Position {
    fn from(value: solver::Position) -> Self {
        Position {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Velocity {
    x: i64,
    y: i64,
}

impl From<solver::Velocity> for Velocity {
    fn from(value: solver::Velocity) -> Self {
        Velocity {
            x: value.x,
            y: value.y,
        }
    }
}

impl Robot {
    #[allow(dead_code)]
    fn step(&self, size: Size) -> Robot {
        Robot {
            position: (self.position + self.velocity) % size,
            velocity: self.velocity,
        }
    }

    fn step_n(&self, n: i64, size: Size) -> Robot {
        Robot {
            position: (self.position + self.velocity * n) % size,
            velocity: self.velocity,
        }
    }
}

impl std::ops::Add<Velocity> for Position {
    type Output = Position;

    fn add(self, rhs: Velocity) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: i64) -> Velocity {
        Velocity {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Rem<Size> for Position {
    type Output = Position;

    fn rem(self, rhs: Size) -> Position {
        Position {
            x: self.x.rem_euclid(rhs.width),
            y: self.y.rem_euclid(rhs.height),
        }
    }
}

struct Debugger {
    size: Size,
    locations: HashMap<Position, usize>,
}

impl Debugger {
    pub fn new(size: Size) -> Self {
        Debugger {
            size,
            locations: HashMap::new(),
        }
    }

    pub fn add(&mut self, robot: Robot) {
        *self.locations.entry(robot.position).or_insert(0) += 1;
    }
}

impl std::fmt::Display for Debugger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let position = Position { x, y };
                let count = self.locations.get(&position).copied().unwrap_or(0);

                if count == 0 {
                    if x == self.size.middle_x() && y == self.size.middle_y() {
                        write!(f, "+")?;
                    } else if x == self.size.middle_x() {
                        write!(f, "|")?;
                    } else if y == self.size.middle_y() {
                        write!(f, "-")?;
                    } else {
                        write!(f, ".")?;
                    }
                } else {
                    write!(f, "{}", count)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct TreeDetector {
    positions: HashSet<Position>,
}

impl TreeDetector {
    fn add(&mut self, robot: Robot) {
        self.positions.insert(robot.position);
    }

    fn score(&self) -> u64 {
        let mut score = 0;
        let up = Velocity { x: 0, y: -1 };
        let down = Velocity { x: 0, y: 1 };
        let left = Velocity { x: -1, y: 0 };
        let right = Velocity { x: 1, y: 0 };

        for position in &self.positions {
            if self.positions.contains(&(*position + up)) {
                score += 1;
            }
            if self.positions.contains(&(*position + down)) {
                score += 1;
            }
            if self.positions.contains(&(*position + left)) {
                score += 1;
            }
            if self.positions.contains(&(*position + right)) {
                score += 1;
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_n() {
        let robot = Robot {
            position: Position { x: 0, y: 4 },
            velocity: Velocity { x: 3, y: -3 },
        };
        let size = Size {
            width: 11,
            height: 7,
        };
        let robot1 = robot.step(size).step(size).step(size).step(size);
        let robot2 = robot.step_n(4, size);
        assert_eq!(robot1, robot2);
    }

    #[test]
    fn test_teleportation() {
        let robot = Robot {
            position: Position { x: 2, y: 4 },
            velocity: Velocity { x: 2, y: -3 },
        };
        let size = Size {
            width: 11,
            height: 7,
        };

        dbg!((1 + -3) % 7);
        dbg!(-2 % 7);

        for i in 0..=5 {
            let mut debugger = Debugger::new(size);
            let robot = robot.step_n(i, size);
            debugger.add(robot);
            println!("After {i} steps:");
            println!("{debugger}");
        }
    }
}
