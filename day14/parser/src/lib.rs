#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    type Runner = Runner;
}

struct Runner(bindings::aoc2024::day14::solver::Input);

impl bindings::exports::aoc::base::day::GuestRunner for Runner {
    fn new(input: String) -> Self {
        let robots = input
            .lines()
            .map(|line| {
                let (p, v) = line.split_once(' ').unwrap();
                let p = p.strip_prefix("p=").unwrap();
                let v = v.strip_prefix("v=").unwrap();
                let (px, py) = p.split_once(',').unwrap();
                let (vx, vy) = v.split_once(',').unwrap();
                let px = px.parse().unwrap();
                let py = py.parse().unwrap();
                let vx = vx.parse().unwrap();
                let vy = vy.parse().unwrap();
                let position = bindings::aoc2024::day14::solver::Position { x: px, y: py };
                let velocity = bindings::aoc2024::day14::solver::Velocity { x: vx, y: vy };
                let robot = bindings::aoc2024::day14::solver::Robot { position, velocity };
                robot
            })
            .collect::<Vec<_>>();

        let size = if robots.len() <= 12 {
            bindings::aoc2024::day14::solver::Size {
                width: 11,
                height: 7,
            }
        } else {
            bindings::aoc2024::day14::solver::Size {
                width: 101,
                height: 103,
            }
        };

        let input = bindings::aoc2024::day14::solver::Input { robots, size };

        Runner(input)
    }

    fn solve_a(&self) -> String {
        bindings::aoc2024::day14::solver::solve_a(&self.0).to_string()
    }

    fn solve_b(&self) -> String {
        bindings::aoc2024::day14::solver::solve_b(&self.0).to_string()
    }
}

bindings::export!(Component with_types_in bindings);
