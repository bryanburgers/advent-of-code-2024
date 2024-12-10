use bindings::aoc2024::day04::solver::Letter;

#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, String) {
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

        let result_a = bindings::aoc2024::day04::solver::solve_a(&input);
        let result_b = bindings::aoc2024::day04::solver::solve_b(&input);

        (result_a.to_string(), result_b.to_string())
    }
}

bindings::export!(Component with_types_in bindings);
