#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(bindings::aoc2024::day15::solver::Input);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let mut lines = input.lines();

        let mut lantern_fish = None;
        let mut tiles = Vec::new();

        let mut y = 0;
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                break;
            }

            for (x, byte) in line.trim().bytes().enumerate() {
                match byte {
                    b'.' => {}
                    b'#' => {
                        tiles.push(bindings::aoc2024::day15::solver::Tile {
                            position: bindings::aoc2024::day15::solver::Position {
                                x: x as i64,
                                y: y as i64,
                            },
                            type_: bindings::aoc2024::day15::solver::TileType::Wall,
                        });
                    }
                    b'O' => {
                        tiles.push(bindings::aoc2024::day15::solver::Tile {
                            position: bindings::aoc2024::day15::solver::Position {
                                x: x as i64,
                                y: y as i64,
                            },
                            type_: bindings::aoc2024::day15::solver::TileType::Box,
                        });
                    }
                    b'@' => {
                        lantern_fish = Some(bindings::aoc2024::day15::solver::Position {
                            x: x as i64,
                            y: y as i64,
                        });
                    }
                    _ => {}
                }
            }

            y += 1;
        }

        let mut moves = Vec::new();

        for line in lines {
            for byte in line.trim().bytes() {
                match byte {
                    b'<' => moves.push(bindings::aoc2024::day15::solver::Move::West),
                    b'^' => moves.push(bindings::aoc2024::day15::solver::Move::North),
                    b'>' => moves.push(bindings::aoc2024::day15::solver::Move::East),
                    b'v' | b'V' => moves.push(bindings::aoc2024::day15::solver::Move::South),
                    _ => {}
                }
            }
        }

        let input = bindings::aoc2024::day15::solver::Input {
            lantern_fish: lantern_fish.unwrap(),
            tiles,
            moves,
        };

        Runner(input)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day15::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day15::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
