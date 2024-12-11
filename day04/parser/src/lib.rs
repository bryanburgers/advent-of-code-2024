use bindings::aoc2024::day04::solver::Letter;

#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(Vec<Vec<Letter>>);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let input = input
            .lines()
            .map(|line| {
                line.trim()
                    .bytes()
                    .map(|b| match b {
                        b'X' => Letter::X,
                        b'M' => Letter::M,
                        b'A' => Letter::A,
                        b'S' => Letter::S,
                        _ => panic!("Invalid letter"),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Letter>>>();
        Runner(input)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day04::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day04::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
