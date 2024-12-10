#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, String) {
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

        let result_a = bindings::aoc2024::day06::solver::solve_a(&area, position);
        let result_b = bindings::aoc2024::day06::solver::solve_b(&area, position);

        (result_a.to_string(), result_b.to_string())
    }
}

bindings::export!(Component with_types_in bindings);
