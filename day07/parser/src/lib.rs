#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, Option<String>) {
        let input = input
            .lines()
            .map(|line| {
                let (test_value, parameters) = line.split_once(": ").unwrap();
                let test_value = test_value.parse().unwrap();
                let parameters = parameters
                    .split(" ")
                    .map(|param| param.parse().unwrap())
                    .collect();
                bindings::aoc2024::day07::solver::CalibrationEquation {
                    test_value,
                    parameters,
                }
            })
            .collect::<Vec<_>>();

        let result_a = bindings::aoc2024::day07::solver::solve_a(&input);
        let result_b = bindings::aoc2024::day07::solver::solve_b(&input);

        (result_a.to_string(), Some(result_b.to_string()))
    }
}

bindings::export!(Component with_types_in bindings);
