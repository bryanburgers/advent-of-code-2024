#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, Option<String>) {
        let result_a = bindings::aoc2024::day03::solver::solve_a(&input);
        (result_a.to_string(), None)
    }
}

bindings::export!(Component with_types_in bindings);
