#[allow(warnings)]
mod bindings;

struct Component;

impl bindings::exports::aoc::base::day::Guest for Component {
    fn run(input: String) -> (String, String) {
        let mut map = Vec::new();

        for line in input.lines() {
            let row = line.bytes().map(|byte| byte - b'0').collect();
            map.push(row);
        }

        let input = bindings::aoc2024::day10::types::TopographicalMap::new(&map);

        let result_a = bindings::aoc2024::day10::solver::solve_a(&input);
        let result_b = bindings::aoc2024::day10::solver::solve_b(&input);

        (result_a.to_string(), result_b.to_string())
    }
}

struct MyTopographicalMap(Vec<Vec<u8>>);

impl bindings::exports::aoc2024::day10::types::Guest for Component {
    type TopographicalMap = MyTopographicalMap;
}

impl bindings::exports::aoc2024::day10::types::GuestTopographicalMap for MyTopographicalMap {
    fn new(map: Vec<Vec<u8>>) -> Self {
        MyTopographicalMap(map)
    }

    fn map_width(&self) -> u32 {
        self.0[0].len() as u32
    }

    fn map_height(&self) -> u32 {
        self.0.len() as u32
    }

    fn height_at_location(&self, x: u32, y: u32) -> u8 {
        self.0[y as usize][x as usize]
    }
}

bindings::export!(Component with_types_in bindings);
