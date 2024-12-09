use clap::Parser;
use std::path::PathBuf;

wasmtime::component::bindgen!("day-world" in "../wit");

#[derive(Parser)]
struct Args {
    /// The path to the WebAssembly component file that implements the day
    wasm: PathBuf,
    /// The path to the input string
    input: PathBuf,
}

struct Debug;

impl aoc::base::debug::Host for Debug {
    fn info(&mut self, message: String) {
        for (idx, line) in message.lines().enumerate() {
            if idx == 0 {
                println!("\x1b[36m\x1b[2minfo:\x1b[0m\x1b[36m {}\x1b[0m", line);
            } else {
                println!("\x1b[36m\x1b[2m ... \x1b[0m\x1b[36m {}\x1b[0m", line);
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let bytes = std::fs::read(args.wasm)?;
    let input = std::fs::read_to_string(args.input)?;

    let engine = wasmtime::Engine::default();
    let mut linker = wasmtime::component::Linker::new(&engine);
    DayWorld::add_to_linker(&mut linker, |state: &mut Debug| state)?;
    let mut store = wasmtime::Store::new(&engine, Debug);
    let component = wasmtime::component::Component::new(&engine, &bytes)?;
    let instance = DayWorld::instantiate(&mut store, &component, &linker)?;

    let (part1, part2) = instance.aoc_base_day().call_run(&mut store, &input)?;
    println!("{part1}");
    if let Some(part2) = part2 {
        println!("{part2}");
    }

    Ok(())
}
