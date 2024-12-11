#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(Vec<(i32, i32)>);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let input = input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let a = iter.next().unwrap().parse().unwrap();
                let b = iter.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect::<Vec<_>>();
        Runner(input)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day01::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day01::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
