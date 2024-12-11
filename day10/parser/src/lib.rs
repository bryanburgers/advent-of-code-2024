#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(Vec<Vec<u8>>);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let mut map = Vec::new();

        for line in input.lines() {
            let row = line.bytes().map(|byte| byte - b'0').collect();
            map.push(row);
        }

        Runner(map)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day10::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day10::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
