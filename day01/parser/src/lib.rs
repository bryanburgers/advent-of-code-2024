#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, Option<String>) {
        let input = input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let a = iter.next().unwrap().parse().unwrap();
                let b = iter.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect::<Vec<_>>();

        let result_a = bindings::aoc2024::day01::solver::solve_a(&input);
        let result_b = bindings::aoc2024::day01::solver::solve_b(&input);
        (result_a.to_string(), Some(result_b.to_string()))
    }
}

bindings::export!(Component with_types_in bindings);
