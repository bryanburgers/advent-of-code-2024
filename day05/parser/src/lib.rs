use bindings::aoc2024::day05::solver::Input;

#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, String) {
        let mut page_ordering_rules = Vec::new();
        let mut updates = Vec::new();

        let mut lines = input.lines();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let (a, b) = line.split_once('|').unwrap();
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            page_ordering_rules.push((a, b));
        }

        for line in lines {
            let update = line.split(",").map(|val| val.parse().unwrap()).collect();
            updates.push(update);
        }

        let input = Input {
            page_ordering_rules,
            updates,
        };

        let result_a = bindings::aoc2024::day05::solver::solve_a(&input);
        let result_b = bindings::aoc2024::day05::solver::solve_b(&input);

        (result_a.to_string(), result_b.to_string())
    }
}

bindings::export!(Component with_types_in bindings);
