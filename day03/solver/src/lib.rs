use regex::Regex;

#[allow(warnings)]
mod bindings;

struct Component;

#[allow(unused_macros)]
macro_rules! info {
    ($($arg:tt)*) => {
        bindings::aoc::base::debug::info(&format!($($arg)*));
    };
}

impl bindings::exports::aoc2024::day03::solver::Guest for Component {
    fn solve_a(input: String) -> i32 {
        let mut accum = 0;
        let regex = Regex::new(r#"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)"#).unwrap();
        let captures = regex.captures_iter(&input);
        for capture in captures {
            let first = capture
                .name("first")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let second = capture
                .name("second")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            accum += first * second;
        }

        accum
    }
}

bindings::export!(Component with_types_in bindings);
