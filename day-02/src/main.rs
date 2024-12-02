use std::ops::BitOr;

fn main() -> anyhow::Result<()> {
    let input = std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow::anyhow!("input required"))?;
    let input = std::fs::read_to_string(input)?;

    let mut count = 0;
    for line in input.trim().lines() {
        fn is_proper_distance(a: i32, b: i32) -> bool {
            let diff = a.abs_diff(b);
            diff >= 1 && diff <= 3
        }

        let parts = line
            .split_whitespace()
            .map(|word| word.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (first, rest) = parts.split_first().unwrap();
        let (_, valid, direction) = rest.into_iter().fold(
            (*first, true, Direction::Either),
            |(previous, valid, direction), &current| {
                let direction = direction | Direction::from_pair(previous, current);
                let valid = valid && is_proper_distance(previous, current);
                (current, valid, direction)
            },
        );
        if valid && matches!(direction, Direction::Increasing | Direction::Decreasing) {
            count += 1;
        }
    }
    println!("{count}");

    Ok(())
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

impl BitOr for Direction {
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
