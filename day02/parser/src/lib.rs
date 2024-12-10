#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, String) {
        let input = input
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|word| word.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let result_a = bindings::aoc2024::day02::solver::solve_a(&input);
        let result_b = bindings::aoc2024::day02::solver::solve_b(&input);
        (result_a.to_string(), result_b.to_string())
    }
}

bindings::export!(Component with_types_in bindings);
