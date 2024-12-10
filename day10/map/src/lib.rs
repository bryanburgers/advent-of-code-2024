#[allow(warnings)]
mod bindings;

struct Component;

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
