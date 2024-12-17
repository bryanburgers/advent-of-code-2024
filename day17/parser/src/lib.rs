#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(bindings::aoc2024::day17::solver::Machine);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let mut lines = input.lines();
        let register_a = lines
            .next()
            .unwrap()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap();
        let register_b = lines
            .next()
            .unwrap()
            .strip_prefix("Register B: ")
            .unwrap()
            .parse()
            .unwrap();
        let register_c = lines
            .next()
            .unwrap()
            .strip_prefix("Register C: ")
            .unwrap()
            .parse()
            .unwrap();
        let _blank_line = lines.next().unwrap();
        let program = lines
            .next()
            .unwrap()
            .strip_prefix("Program: ")
            .unwrap()
            .trim()
            .split(',')
            .map(|instruction| instruction.parse().unwrap())
            .collect();
        let input = bindings::aoc2024::day17::solver::Machine {
            register_a,
            register_b,
            register_c,
            program,
        };

        Runner(input)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day17::solver::solve_a(&self.0)
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day17::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
