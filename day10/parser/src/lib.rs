#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, String) {
        let mut map = Vec::new();

        for line in input.lines() {
            let row = line.bytes().map(|byte| byte - b'0').collect();
            map.push(row);
        }

        let result_a = bindings::aoc2024::day10::solver::solve_a(&map);
        let result_b = bindings::aoc2024::day10::solver::solve_b(&map);

        (result_a.to_string(), result_b.to_string())
    }
}

bindings::export!(Component with_types_in bindings);
