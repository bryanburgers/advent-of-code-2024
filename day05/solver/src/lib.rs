use std::collections::HashSet;

use bindings::exports::aoc2024::day05::solver;

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
    fn solve_a(input: solver::Input) -> i32 {
        let input = Input::from(input);
        let mut accum = 0;

        for update in &input.updates {
            let is_valid = input.is_valid_update(update);

            if is_valid {
                accum += update.middle();
            }
        }
        accum
    }

    fn solve_b(input: solver::Input) -> i32 {
        let input = Input::from(input);
        let mut accum = 0;

        for update in &input.updates {
            if input.is_valid_update(update) {
                continue;
            }

            let sorted = update.sorted(&input);
            if !input.is_valid_update(&sorted) {
                info!("still not valid {:?} -> {:?}", update.0, sorted.0);
            }
            accum += sorted.middle();
        }
        accum
    }
}

bindings::export!(Component with_types_in bindings);

struct Input {
    page_ordering_rules: HashSet<(i32, i32)>,
    updates: Vec<Update>,
}

impl Input {
    pub fn is_valid_page_pair(&self, update_pair: (i32, i32)) -> bool {
        self.page_ordering_rules.contains(&update_pair)
    }

    pub fn is_valid_update(&self, update: &Update) -> bool {
        update.pairs().all(|pair| self.is_valid_page_pair(pair))
    }

    pub fn sort(&self, left: i32, right: i32) -> std::cmp::Ordering {
        if self.page_ordering_rules.contains(&(left, right)) {
            return std::cmp::Ordering::Less;
        } else if self.page_ordering_rules.contains(&(right, left)) {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    }
}

impl From<solver::Input> for Input {
    fn from(input: solver::Input) -> Self {
        Input {
            page_ordering_rules: input.page_ordering_rules.into_iter().collect(),
            updates: input.updates.into_iter().map(Update::from).collect(),
        }
    }
}

struct Update(Vec<i32>);

impl From<Vec<i32>> for Update {
    fn from(input: Vec<i32>) -> Self {
        Update(input)
    }
}

impl Update {
    pub fn middle(&self) -> i32 {
        self.0[self.0.len() / 2]
    }

    pub fn pairs(&self) -> UpdatePairIterator {
        UpdatePairIterator {
            updates: self,
            i: 0,
            j: 1,
        }
    }

    pub fn sorted(&self, input: &Input) -> Self {
        let mut values = self.0.clone();
        values.sort_by(|a, b| input.sort(*a, *b));
        Self(values)
    }
}

pub struct UpdatePairIterator<'a> {
    updates: &'a Update,
    i: usize,
    j: usize,
}

impl Iterator for UpdatePairIterator<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.updates.0.len() || self.j >= self.updates.0.len() {
            return None;
        }

        let pair = (self.updates.0[self.i], self.updates.0[self.j]);

        self.j += 1;
        if self.j >= self.updates.0.len() {
            self.i += 1;
            self.j = self.i + 1;
        }

        Some(pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_middle() {
        let update = Update::from(vec![75, 47, 61, 53, 29]);
        assert_eq!(update.middle(), 61);
    }

    #[test]
    fn update_pairs() {
        let update = Update::from(vec![1, 2, 3]);
        let mut pairs = update.pairs();
        assert_eq!(pairs.next(), Some((1, 2)));
        assert_eq!(pairs.next(), Some((1, 3)));
        assert_eq!(pairs.next(), Some((2, 3)));
        assert_eq!(pairs.next(), None);
        let update = Update::from(vec![75, 47, 61, 53, 29]);
        let mut pairs = update.pairs();
        assert_eq!(pairs.next(), Some((75, 47)));
        assert_eq!(pairs.next(), Some((75, 61)));
        assert_eq!(pairs.count(), 8)
    }
}
