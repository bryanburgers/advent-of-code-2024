use bindings::exports::aoc2024::day13::solver;

#[allow(warnings)]
mod bindings;

struct Component;

#[allow(unused_macros)]
macro_rules! info {
    ($($arg:tt)*) => {
        bindings::aoc::base::debug::info(&format!($($arg)*));
    };
}

impl solver::Guest for Component {
    fn solve_a(input: Vec<solver::Machine>) -> u64 {
        let mut accum = 0;
        for machine in input {
            if let Some((a, b)) = machine.solve() {
                let cost = a * 3 + b;
                accum += cost;
            }
        }
        accum
    }

    fn solve_b(_input: Vec<solver::Machine>) -> u64 {
        0
    }
}

bindings::export!(Component with_types_in bindings);

impl solver::Machine {
    fn solve(&self) -> Option<(u64, u64)> {
        // Ax * a + Bx * b = Px
        // Ay * a + By * b = Py

        // Multiply both equations by Ax and Ay, respectively
        // Ax * Ay * a + Bx * Ay * b = Px * Ay
        // Ax * Ay * b + By * Ax * b = Py * Ax

        // Subtracting those two equations
        // (Bx * Ay * b) - (By * Ax * b) = (Px * Ay) - (Py * Ax)

        // Distibuting the shared b factor
        // b * (Bx * Ay - By * Ax) = (Px * Ay) - (Py * Ax)

        // And dividing
        // b = ((Px * Ay) - (Py * Ax)) / (Bx * Ay - By * Ax)

        let b = ((self.prize.x * self.button_a.y) - (self.prize.y * self.button_a.x))
            / (self.button_b.x * self.button_a.y - self.button_b.y * self.button_a.x);

        // Now we can substitute b back into the first equation
        // Ax * a + Bx * b = Px

        // Ax * a = Px - (Bx * b)

        // a = (Px - (Bx * b)) / Ax

        let a = (self.prize.x - (self.button_b.x * b)) / self.button_a.x;

        // And finally, we need to make sure the solution is valid
        if self.button_a.x * a + self.button_b.x * b == self.prize.x
            && self.button_a.y * a + self.button_b.y * b == self.prize.y
        {
            Some((a as u64, b as u64))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_example_1() {
        let machine = solver::Machine {
            button_a: solver::ButtonPress { x: 94, y: 34 },
            button_b: solver::ButtonPress { x: 22, y: 67 },
            prize: solver::Position { x: 8400, y: 5400 },
        };
        assert_eq!(machine.solve(), Some((80, 40)));
    }

    #[test]
    fn test_machine_example_2() {
        let machine = solver::Machine {
            button_a: solver::ButtonPress { x: 26, y: 66 },
            button_b: solver::ButtonPress { x: 67, y: 21 },
            prize: solver::Position { x: 12748, y: 12176 },
        };
        assert_eq!(machine.solve(), None);
    }
}
