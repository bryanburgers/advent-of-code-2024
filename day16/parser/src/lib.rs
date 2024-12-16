#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(Vec<Vec<bindings::aoc2024::day16::solver::Tile>>);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let input = input
            .lines()
            .map(|line| {
                line.trim()
                    .bytes()
                    .map(|byte| match byte {
                        b'.' => bindings::aoc2024::day16::solver::Tile::Open,
                        b'#' => bindings::aoc2024::day16::solver::Tile::Wall,
                        b'S' => bindings::aoc2024::day16::solver::Tile::Start,
                        b'E' => bindings::aoc2024::day16::solver::Tile::End,
                        b => panic!("{b} unexpected"),
                    })
                    .collect()
            })
            .collect();

        Runner(input)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day16::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day16::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
