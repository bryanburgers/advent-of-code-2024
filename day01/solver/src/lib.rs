#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc2024::day01::solver::Guest for Component {
    fn solve_a(input: Vec<(i32, i32)>) -> i32 {
        let (mut a, mut b) = input.into_iter().unzip::<i32, i32, Vec<_>, Vec<_>>();
        a.sort();
        b.sort();

        a.into_iter()
            .zip(b.into_iter())
            .map(|(a, b)| a.abs_diff(b) as i32)
            .sum()
    }

    fn solve_b(input: Vec<(i32, i32)>) -> i32 {
        let (a, b) = input.into_iter().unzip::<i32, i32, Vec<_>, Vec<_>>();
        let frequency = b
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut frequency, b| {
                *frequency.entry(b).or_insert(0) += 1;
                frequency
            });

        a.into_iter()
            .map(|a| a * *frequency.get(&a).unwrap_or(&0))
            .sum()
    }
}

bindings::export!(Component with_types_in bindings);
