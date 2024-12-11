#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(Vec<Vec<bool>>, (u32, u32));

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let mut area = Vec::new();
        let mut position = None;

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => row.push(false),
                    '#' => row.push(true),
                    '^' => {
                        row.push(false);
                        position = Some((x as u32, y as u32));
                    }
                    _ => panic!("unexpected char"),
                }
            }

            area.push(row);
        }

        let position = position.unwrap();

        Runner(area, position)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day06::solver::solve_a(&self.0, self.1).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day06::solver::solve_b(&self.0, self.1).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
