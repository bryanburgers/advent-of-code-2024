#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(Vec<bindings::aoc2024::day07::solver::CalibrationEquation>);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
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

        Runner(input)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day07::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day07::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
