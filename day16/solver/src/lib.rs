use std::collections::HashMap;

use bindings::exports::aoc2024::day16::solver::{self};

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
    fn solve_a(input: Vec<Vec<solver::Tile>>) -> u64 {
        let map = Map::from(input);
        map.solve()
    }

    fn solve_b(_input: Vec<Vec<solver::Tile>>) -> u64 {
        0
    }
}

bindings::export!(Component with_types_in bindings);

struct Map {
    values: Vec<Vec<solver::Tile>>,
    width: i64,
    height: i64,
    start: Position,
    end: Position,
}

impl Map {
    #[allow(dead_code)]
    fn width(&self) -> i64 {
        self.width
    }
    #[allow(dead_code)]
    fn height(&self) -> i64 {
        self.height
    }
    fn start(&self) -> Position {
        self.start
    }
    fn end(&self) -> Position {
        self.end
    }
    fn is_open(&self, position: Position) -> bool {
        match self.values[position.y as usize][position.x as usize] {
            solver::Tile::Open => true,
            solver::Tile::Start => true,
            solver::Tile::End => true,
            _ => false,
        }
    }
    fn solve(&self) -> u64 {
        let mut visited: HashMap<Node, bool> = HashMap::default();
        let mut least: HashMap<Node, u64> = HashMap::new();
        let start_node = Node {
            position: self.start(),
            orientation: Orientation::East,
        };
        least.insert(start_node, 0);
        visited.insert(start_node, false);

        fn find_least_unvisited(
            least: &HashMap<Node, u64>,
            visited: &HashMap<Node, bool>,
        ) -> Option<Node> {
            let mut l = None;
            for (node, val) in least {
                if visited.get(&node).is_some_and(|val| *val) {
                    continue;
                }
                let Some((_, l_val)) = l else {
                    l = Some((node, val));
                    continue;
                };

                if val < l_val {
                    l = Some((node, val));
                }
            }

            l.map(|(node, _)| *node)
        }

        while let Some(node) = find_least_unvisited(&least, &visited) {
            let value = *least.get(&node).unwrap();
            // info!("Evaluating {node:?}, value to get here is {value}");
            visited.insert(node, true);

            if node.position == self.end() {
                return value;
            }

            for (neighbor, cost) in node.neighbors() {
                if !self.is_open(node.position) {
                    continue;
                }

                let new_value = value + cost;
                least
                    .entry(neighbor)
                    .and_modify(|current| {
                        if new_value < *current {
                            *current = new_value;
                        }
                    })
                    .or_insert(new_value);
                visited.entry(neighbor).or_insert(false);
            }
        }

        u64::MAX
    }
}

impl From<Vec<Vec<solver::Tile>>> for Map {
    fn from(values: Vec<Vec<solver::Tile>>) -> Self {
        let mut start = None;
        let mut end = None;
        let width = values[0].len() as i64;
        let height = values.len() as i64;

        for y in 0..height {
            for x in 0..width {
                if values[y as usize][x as usize] == solver::Tile::Start {
                    start = Some(Position { x, y });
                }
                if values[y as usize][x as usize] == solver::Tile::End {
                    end = Some(Position { x, y });
                }
            }
        }

        Map {
            start: start.unwrap(),
            end: end.unwrap(),
            width,
            height,
            values,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn in_direction(self, orientation: Orientation) -> Self {
        match orientation {
            Orientation::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Orientation::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Orientation::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Orientation::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    fn turn_right(self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Node {
    position: Position,
    orientation: Orientation,
}

impl Node {
    fn neighbors(&self) -> [(Node, u64); 3] {
        [
            (
                Node {
                    position: self.position.in_direction(self.orientation),
                    orientation: self.orientation,
                },
                1,
            ),
            (
                Node {
                    position: self.position,
                    orientation: self.orientation.turn_left(),
                },
                1000,
            ),
            (
                Node {
                    position: self.position,
                    orientation: self.orientation.turn_right(),
                },
                1000,
            ),
        ]
    }
}
