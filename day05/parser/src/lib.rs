use bindings::aoc2024::day05::solver::Input;

#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(Input);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
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

        Runner(input)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day05::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day05::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
