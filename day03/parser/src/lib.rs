#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, Option<String>) {
        let result_a = bindings::aoc2024::day03::solver::solve_a(&input);
        let result_b = bindings::aoc2024::day03::solver::solve_b(&input);
        (result_a.to_string(), Some(result_b.to_string()))
    }
}

bindings::export!(Component with_types_in bindings);
