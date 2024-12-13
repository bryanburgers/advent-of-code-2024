#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(Vec<bindings::aoc2024::day13::solver::Machine>);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let mut lines = input.trim().lines();
        let mut machines = Vec::new();
        loop {
            let button_a_line = lines.next().unwrap();
            let button_b_line = lines.next().unwrap();
            let prize_line = lines.next().unwrap();

            let button_a_stripped = button_a_line.strip_prefix("Button A: X+").unwrap();
            let (button_a_x, button_a_y) = button_a_stripped.split_once(", Y+").unwrap();
            let button_a = bindings::aoc2024::day13::solver::ButtonPress {
                x: button_a_x.parse().unwrap(),
                y: button_a_y.parse().unwrap(),
            };

            let button_b_stripped = button_b_line.strip_prefix("Button B: X+").unwrap();
            let (button_b_x, button_b_y) = button_b_stripped.split_once(", Y+").unwrap();
            let button_b = bindings::aoc2024::day13::solver::ButtonPress {
                x: button_b_x.parse().unwrap(),
                y: button_b_y.parse().unwrap(),
            };

            let prize_stripped = prize_line.strip_prefix("Prize: X=").unwrap();
            let (prize_x, prize_y) = prize_stripped.split_once(", Y=").unwrap();
            let prize = bindings::aoc2024::day13::solver::Position {
                x: prize_x.parse().unwrap(),
                y: prize_y.parse().unwrap(),
            };

            let machine = bindings::aoc2024::day13::solver::Machine {
                button_a,
                button_b,
                prize,
            };
            machines.push(machine);

            // Blank line separator
            if lines.next().is_none() {
                break;
            }
        }

        Runner(machines)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day13::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day13::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
