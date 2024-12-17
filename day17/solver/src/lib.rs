use std::collections::VecDeque;

use bindings::exports::aoc2024::day17::solver::{self};

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
    fn solve_a(input: solver::Machine) -> Vec<u8> {
        let mut machine = Machine::from(input);
        machine.run()
    }

    fn solve_b(input: solver::Machine) -> u64 {
        let machine = Machine::from(input);

        let mut options = VecDeque::from([Words::from(0)]);

        while let Some(option) = options.pop_front() {
            for i in 0..8 {
                let a = option.append_digit(i);
                let mut machine = machine.clone();
                machine.register_a = a.into();
                let output = machine.run();

                // info!(
                //     "A={a} output={}",
                //     OutputDiff(
                //         &output,
                //         &machine.program[machine.program.len() - output.len()..]
                //     ),
                // );
                if &output == &machine.program {
                    return a.into();
                }
                if &output == &machine.program[machine.program.len() - output.len()..] {
                    options.push_back(a);
                }
            }
        }

        0
    }
}

bindings::export!(Component with_types_in bindings);

#[derive(Debug, Clone)]
struct Machine {
    ip: usize,
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<u8>,
}

impl From<solver::Machine> for Machine {
    fn from(machine: solver::Machine) -> Self {
        Machine {
            ip: 0,
            register_a: machine.register_a,
            register_b: machine.register_b,
            register_c: machine.register_c,
            program: machine.program,
        }
    }
}

impl Machine {
    fn is_halted(&self) -> bool {
        self.ip >= self.program.len()
    }

    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while !self.is_halted() {
            if let Some(out) = self.step_one() {
                output.push(out);
            }
        }

        output
    }

    fn step_one(&mut self) -> Option<u8> {
        let Some(instruction) = self.program.get(self.ip) else {
            return None;
        };

        let mut ret = None;
        match instruction {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => self.jnz(),
            4 => self.bxc(),
            5 => ret = Some(self.out()),
            6 => self.bdv(),
            7 => self.cdv(),
            _ => panic!("Unexpected instruction {instruction}"),
        }

        ret
    }

    fn literal_operand(&self) -> u64 {
        self.program[self.ip + 1] as u64
    }

    fn combo_operand(&self) -> u64 {
        match self.program[self.ip + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            v => panic!("Unexpected combo operand {v}"),
        }
    }

    fn adv(&mut self) {
        let numerator = self.register_a;
        let denominator = self.combo_operand();
        let result = numerator >> denominator;
        self.register_a = result;
        self.ip += 2;
    }

    fn bxl(&mut self) {
        self.register_b = self.register_b ^ self.literal_operand();
        self.ip += 2;
    }

    fn bst(&mut self) {
        self.register_b = self.combo_operand() % 8;
        self.ip += 2;
    }

    fn jnz(&mut self) {
        if self.register_a == 0 {
            self.ip += 2;
        } else {
            self.ip = self.literal_operand() as usize;
        }
    }

    fn bxc(&mut self) {
        self.register_b = self.register_b ^ self.register_c;
        self.ip += 2;
    }

    fn out(&mut self) -> u8 {
        let out = (self.combo_operand() % 8) as u8;
        self.ip += 2;
        out
    }

    fn bdv(&mut self) {
        let numerator = self.register_a;
        let denominator = self.combo_operand();
        let result = numerator >> denominator;
        self.register_b = result;
        self.ip += 2;
    }

    fn cdv(&mut self) {
        let numerator = self.register_a;
        let denominator = self.combo_operand();
        let result = numerator >> denominator;
        self.register_c = result;
        self.ip += 2;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Words(u64);

impl Words {
    pub fn append_digit(self, val: u8) -> Self {
        Self((self.0 << 3) + val as u64)
    }
}

impl<const N: usize> From<[u8; N]> for Words {
    fn from(value: [u8; N]) -> Self {
        let mut accum = 0;
        for i in 0..N {
            accum = (accum << 3) + (value[i] as u64);
        }
        Words(accum)
    }
}

impl From<Vec<u8>> for Words {
    fn from(value: Vec<u8>) -> Self {
        let mut accum = 0;
        for v in value {
            accum = (accum << 3) + (v as u64);
        }
        Words(accum)
    }
}

impl From<u64> for Words {
    fn from(value: u64) -> Self {
        Words(value)
    }
}

impl From<Words> for u64 {
    fn from(value: Words) -> Self {
        value.0
    }
}

impl std::fmt::Display for Words {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            write!(f, "0")?;
            return Ok(());
        }

        let mut digits = Vec::new();
        let mut value = self.0;
        while value > 0 {
            digits.push(value % 8);
            value = value >> 3;
        }
        digits.reverse();
        if let Some(width) = f.width() {
            let pad = width.saturating_sub(digits.len());
            for _ in 0..pad {
                f.write_str(" ")?;
            }
        }

        for digit in digits {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
struct OutputDiff<'a>(&'a [u8], &'a [u8]);

impl std::fmt::Display for OutputDiff<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (left, right) in self.0.iter().zip(self.1.iter()) {
            if left == right {
                write!(f, "\x1b[32m{left}")?;
            } else {
                write!(f, "\x1b[31m{left}")?;
            }
        }
        write!(f, "\x1b[0m")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut machine = Machine {
            ip: 0,
            register_a: 0,
            register_b: 0,
            register_c: 9,
            program: vec![2, 6],
        };
        machine.run();
        assert_eq!(machine.register_b, 1);
    }

    #[test]
    fn example_2() {
        let mut machine = Machine {
            ip: 0,
            register_a: 10,
            register_b: 0,
            register_c: 0,
            program: vec![5, 0, 5, 1, 5, 4],
        };
        let output = machine.run();
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn example_3() {
        let mut machine = Machine {
            ip: 0,
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let output = machine.run();
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(machine.register_a, 0);
    }

    #[test]
    fn example_4() {
        let mut machine = Machine {
            ip: 0,
            register_a: 0,
            register_b: 29,
            register_c: 0,
            program: vec![1, 7],
        };
        machine.run();
        assert_eq!(machine.register_b, 26);
    }

    #[test]
    fn example_5() {
        let mut machine = Machine {
            ip: 0,
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            program: vec![4, 0],
        };
        machine.run();
        assert_eq!(machine.register_b, 44354);
    }
}
