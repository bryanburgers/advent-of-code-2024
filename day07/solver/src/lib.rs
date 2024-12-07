use bindings::exports::aoc2024::day07::solver;

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
    fn solve_a(input: Vec<solver::CalibrationEquation>) -> u64 {
        let mut accum = 0;
        for equation in input {
            let equation = CalibrationEquation::from(equation);
            if equation.can_be_true() {
                accum += equation.test_value;
            }
        }

        accum
    }

    fn solve_b(input: Vec<solver::CalibrationEquation>) -> u64 {
        let mut accum = 0;
        for equation in input {
            let equation = CalibrationEquation::from(equation);
            if equation.can_be_true_second() {
                accum += equation.test_value;
            }
        }

        accum
    }
}

bindings::export!(Component with_types_in bindings);

struct CalibrationEquation {
    test_value: u64,
    parameters: Vec<u64>,
}

impl From<solver::CalibrationEquation> for CalibrationEquation {
    fn from(input: solver::CalibrationEquation) -> Self {
        CalibrationEquation {
            test_value: input.test_value,
            parameters: input.parameters,
        }
    }
}

impl CalibrationEquation {
    pub fn can_be_true(&self) -> bool {
        struct StackItem<'a> {
            current_value: u64,
            remaining_parameters: &'a [u64],
        }
        let mut stack = Vec::new();

        if let Some((left, right)) = self.parameters.split_first() {
            stack.push(StackItem {
                current_value: *left,
                remaining_parameters: right,
            });
        }

        while let Some(stack_item) = stack.pop() {
            let StackItem {
                current_value,
                remaining_parameters,
            } = stack_item;

            if remaining_parameters.is_empty() {
                if current_value == self.test_value {
                    return true;
                }
                continue;
            }

            if let Some((next, remaining)) = remaining_parameters.split_first() {
                let mul_result = current_value * next;
                let add_result = current_value + next;

                if mul_result == self.test_value && remaining.is_empty() {
                    return true;
                }
                if add_result == self.test_value && remaining.is_empty() {
                    return true;
                }

                stack.push(StackItem {
                    current_value: mul_result,
                    remaining_parameters: remaining,
                });
                stack.push(StackItem {
                    current_value: add_result,
                    remaining_parameters: remaining,
                });
            }
        }

        false
    }

    pub fn can_be_true_second(&self) -> bool {
        struct StackItem<'a> {
            current_value: u64,
            remaining_parameters: &'a [u64],
        }
        let mut stack = Vec::new();

        if let Some((left, right)) = self.parameters.split_first() {
            stack.push(StackItem {
                current_value: *left,
                remaining_parameters: right,
            });
        }

        while let Some(stack_item) = stack.pop() {
            let StackItem {
                current_value,
                remaining_parameters,
            } = stack_item;

            if remaining_parameters.is_empty() {
                if current_value == self.test_value {
                    return true;
                }
                continue;
            }

            if let Some((next, remaining)) = remaining_parameters.split_first() {
                let mul_result = current_value * next;
                let add_result = current_value + next;
                let con_result = concat(current_value, *next);

                if mul_result == self.test_value && remaining.is_empty() {
                    return true;
                }
                if add_result == self.test_value && remaining.is_empty() {
                    return true;
                }
                if con_result == self.test_value && remaining.is_empty() {
                    return true;
                }

                stack.push(StackItem {
                    current_value: mul_result,
                    remaining_parameters: remaining,
                });
                stack.push(StackItem {
                    current_value: add_result,
                    remaining_parameters: remaining,
                });
                stack.push(StackItem {
                    current_value: con_result,
                    remaining_parameters: remaining,
                });
            }
        }

        false
    }
}

fn concat(left: u64, right: u64) -> u64 {
    if right == 0 {
        return left * 10;
    }

    let mut multiplier = 10;
    loop {
        if right > multiplier {
            multiplier *= 10;
            continue;
        } else {
            return left * multiplier + right;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration() {
        let equation = CalibrationEquation {
            test_value: 3267,
            parameters: vec![81, 40, 27],
        };
        assert!(equation.can_be_true());
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 2), 12);
        assert_eq!(concat(12, 3), 123);
        assert_eq!(concat(123, 4), 1234);
        assert_eq!(concat(12, 345), 12345);
        assert_eq!(concat(12, 0), 120);
    }
}
