#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc2024::day02::solver::Guest for Component {
    fn solve_a(input: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for parts in input {
            if is_valid_part_a(&parts) {
                count += 1;
            }
        }
        count
    }

    fn solve_b(input: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for parts in input {
            if is_valid_part_b_brute_force(&parts) {
                count += 1;
            }
        }
        count
    }
}

fn is_valid_part_a(input: &[i32]) -> bool {
    fn is_proper_distance(a: i32, b: i32) -> bool {
        let diff = a.abs_diff(b);
        diff >= 1 && diff <= 3
    }

    let mut direction = Direction::Either;
    for i in 0..input.len() - 1 {
        let left = input[i];
        let right = input[i + 1];
        if !is_proper_distance(left, right) {
            return false;
        }
        direction = direction | Direction::from_pair(left, right);
    }
    matches!(direction, Direction::Increasing | Direction::Decreasing)
}

fn is_valid_part_b_brute_force(input: &[i32]) -> bool {
    if is_valid_part_a(input) {
        return true;
    }

    for i in 0..input.len() {
        let parts = input
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, x)| *x)
            .collect::<Vec<_>>();
        if is_valid_part_a(&parts) {
            return true;
        }
    }

    false
}

#[derive(Debug)]
enum Direction {
    Either,
    Increasing,
    Decreasing,
    Both,
}

impl Direction {
    fn from_pair(a: i32, b: i32) -> Self {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => Self::Increasing,
            std::cmp::Ordering::Greater => Self::Decreasing,
            std::cmp::Ordering::Equal => Self::Either,
        }
    }
}

impl std::ops::BitOr for Direction {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Direction::Either, Direction::Either) => Direction::Either,
            (Direction::Either, Direction::Increasing) => Direction::Increasing,
            (Direction::Either, Direction::Decreasing) => Direction::Decreasing,
            (Direction::Either, Direction::Both) => Direction::Both,
            (Direction::Increasing, Direction::Either) => Direction::Increasing,
            (Direction::Increasing, Direction::Increasing) => Direction::Increasing,
            (Direction::Increasing, Direction::Decreasing) => Direction::Both,
            (Direction::Increasing, Direction::Both) => Direction::Both,
            (Direction::Decreasing, Direction::Either) => Direction::Decreasing,
            (Direction::Decreasing, Direction::Increasing) => Direction::Both,
            (Direction::Decreasing, Direction::Decreasing) => Direction::Decreasing,
            (Direction::Decreasing, Direction::Both) => Direction::Both,
            (Direction::Both, Direction::Either) => Direction::Both,
            (Direction::Both, Direction::Increasing) => Direction::Both,
            (Direction::Both, Direction::Decreasing) => Direction::Both,
            (Direction::Both, Direction::Both) => Direction::Both,
        }
    }
}

bindings::export!(Component with_types_in bindings);
