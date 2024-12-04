use bindings::exports::aoc2024::day04::solver;

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
    fn solve_a(input: Vec<Vec<solver::Letter>>) -> i32 {
        let board = Board::from(input);
        board.find_xmases() as i32
    }
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<Letter>>,
}

impl From<Vec<Vec<solver::Letter>>> for Board {
    fn from(input: Vec<Vec<solver::Letter>>) -> Self {
        Board {
            board: input
                .into_iter()
                .map(|row| row.into_iter().map(Letter::from).collect())
                .collect(),
        }
    }
}

impl Board {
    fn find_xmases(&self) -> usize {
        let mut accum = 0;
        for x in 0..self.width() {
            for y in 0..self.height() {
                accum += self.find_xmases_at(x, y);
            }
        }
        accum
    }

    fn find_xmases_at(&self, x: usize, y: usize) -> usize {
        let mut accum = 0;
        if self.find_xmas_at(x, y, 1, 0) {
            accum += 1;
        }
        if self.find_xmas_at(x, y, 1, 1) {
            accum += 1;
        }
        if self.find_xmas_at(x, y, 0, 1) {
            accum += 1;
        }
        if self.find_xmas_at(x, y, -1, 1) {
            accum += 1;
        }
        if self.find_xmas_at(x, y, -1, 0) {
            accum += 1;
        }
        if self.find_xmas_at(x, y, -1, -1) {
            accum += 1;
        }
        if self.find_xmas_at(x, y, 0, -1) {
            accum += 1;
        }
        if self.find_xmas_at(x, y, 1, -1) {
            accum += 1;
        }
        accum
    }

    fn find_xmas_at(&self, x: usize, y: usize, dx: i8, dy: i8) -> bool {
        let Some(letters) = self.four_letters(x, y, dx, dy) else {
            return false;
        };
        letters[0].is_x() && letters[1].is_m() && letters[2].is_a() && letters[3].is_s()
    }

    fn width(&self) -> usize {
        self.board[0].len()
    }

    fn height(&self) -> usize {
        self.board.len()
    }

    fn four_letters(&self, x: usize, y: usize, dx: i8, dy: i8) -> Option<[Letter; 4]> {
        let mut letters = [Letter::X; 4];

        let last_x = x as i32 + dx as i32 * 3;
        let last_y = y as i32 + dy as i32 * 3;
        if last_x < 0
            || last_y < 0
            || last_x as usize >= self.width()
            || last_y as usize >= self.height()
        {
            return None;
        }

        let mut x = x;
        let mut y = y;
        for i in 0..4 {
            letters[i] = self.board[y as usize][x as usize];
            x = (x as i32 + dx as i32) as usize;
            y = (y as i32 + dy as i32) as usize;
        }
        Some(letters)
    }
}

#[derive(Copy, Clone, Debug)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl From<solver::Letter> for Letter {
    fn from(value: solver::Letter) -> Self {
        match value {
            solver::Letter::X => Letter::X,
            solver::Letter::M => Letter::M,
            solver::Letter::A => Letter::A,
            solver::Letter::S => Letter::S,
        }
    }
}

impl Letter {
    fn is_x(self) -> bool {
        matches!(self, Letter::X)
    }
    fn is_m(self) -> bool {
        matches!(self, Letter::M)
    }
    fn is_a(self) -> bool {
        matches!(self, Letter::A)
    }
    fn is_s(self) -> bool {
        matches!(self, Letter::S)
    }
}

bindings::export!(Component with_types_in bindings);
