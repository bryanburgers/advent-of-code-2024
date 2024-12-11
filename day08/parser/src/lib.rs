#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(bindings::aoc2024::day08::solver::Input);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut antenas = Vec::new();

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if y > max_y {
                    max_y = y;
                }
                if x > max_x {
                    max_x = x;
                }
                match char {
                    '.' => {}
                    _ => {
                        let position = bindings::aoc2024::day08::solver::Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        let antena = bindings::aoc2024::day08::solver::Antena {
                            position,
                            frequency: char,
                        };
                        antenas.push(antena);
                    }
                }
            }
        }
        let input = bindings::aoc2024::day08::solver::Input {
            area_width: max_x as i32 + 1,
            area_height: max_y as i32 + 1,
            antenas,
        };

        Runner(input)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day08::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day08::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
