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

    fn solve_b(input: String) -> i32 {
        let mut accum = 0;
        let mut enabled = true;
        let regex = match Regex::new(
            r#"(?x)
            (?<do>do)\(\)
            |
            (?<dont>don't)\(\)
            |
            (?<mul>mul)\((?<first>\d{1,3}),(?<second>\d{1,3})\)
        "#,
        ) {
            Ok(x) => x,
            Err(err) => {
                info!("Error creating regex: {err:?}");
                return 0;
            }
        };
        let captures = regex.captures_iter(&input);
        for capture in captures {
            if capture.name("do").is_some() {
                enabled = true;
            } else if capture.name("dont").is_some() {
                enabled = false;
            } else if capture.name("mul").is_some() {
                let Some(first) = capture.name("first") else {
                    info!("{capture:?} didn't have a first");
                    continue;
                };
                let first = first.as_str().parse::<i32>().unwrap();

                let Some(second) = capture.name("second") else {
                    info!("{capture:?} didn't have a second");
                    continue;
                };

                let second = second.as_str().parse::<i32>().unwrap();

                if enabled {
                    accum += first * second;
                }
            }
        }

        accum
    }
}

bindings::export!(Component with_types_in bindings);
