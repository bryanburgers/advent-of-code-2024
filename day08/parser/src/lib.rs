#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, String) {
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
        let result_a = bindings::aoc2024::day08::solver::solve_a(&input);
        let result_b = bindings::aoc2024::day08::solver::solve_b(&input);

        (result_a.to_string(), result_b.to_string())
    }
}

bindings::export!(Component with_types_in bindings);
