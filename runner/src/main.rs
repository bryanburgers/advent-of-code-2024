use std::path::PathBuf;

use clap::Parser;

wasmtime::component::bindgen!("day-world" in "../wit");

#[derive(Parser)]
struct Args {
    /// The path to the WebAssembly component file that implements the day
    wasm: PathBuf,
    /// The path to the input string
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let bytes = std::fs::read(args.wasm)?;
    let input = std::fs::read_to_string(args.input)?;

    let engine = wasmtime::Engine::default();
    let mut store = wasmtime::Store::new(&engine, ());
    let linker = wasmtime::component::Linker::new(&engine);
    let component = wasmtime::component::Component::new(&engine, &bytes)?;
    let instance = DayWorld::instantiate(&mut store, &component, &linker)?;

    let (part1, part2) = instance.aoc_base_day().call_run(&mut store, &input)?;
    println!("{part1}");
    if let Some(part2) = part2 {
        println!("{part2}");
    }

    Ok(())
}
