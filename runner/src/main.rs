use clap::Parser;
use std::{path::PathBuf, time::Instant};

wasmtime::component::bindgen!("day-world" in "../wit");

#[derive(Parser)]
struct Args {
    /// The path to the WebAssembly component file that implements the day
    wasm: PathBuf,
    /// The path to the input string
    input: PathBuf,
}

struct Debug(bool);

impl Debug {
    pub fn reset(&mut self) {
        self.0 = false;
    }

    pub fn set_logged(&mut self) {
        self.0 = true;
    }

    pub fn did_log(&self) -> bool {
        self.0
    }
}

impl aoc::base::debug::Host for Debug {
    fn info(&mut self, message: String) {
        self.set_logged();
        if message.trim().is_empty() {
            println!("");
        }
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
    let mut store = wasmtime::Store::new(&engine, Debug(false));
    let component = wasmtime::component::Component::new(&engine, &bytes)?;
    let instance = DayWorld::instantiate(&mut store, &component, &linker)?;

    let runner = instance.aoc_base_day().runner();
    let resource = runner.call_constructor(&mut store, &input)?;

    println!("\x1b[4m\x1b[36mPart 1\x1b[0m");
    let start = Instant::now();
    let part1 = runner.call_solve_a(&mut store, resource)?;
    let elapsed = start.elapsed();
    if store.data().did_log() {
        println!(
            "\x1b[35m\x1b[2mtook:\x1b[0m\x1b[35m {:1.1?}\x1b[0m",
            elapsed
        );
    } else {
        println!(
            "\x1b[F\x1b[K\x1b[4m\x1b[36mPart 1\x1b[0m \x1b[35m{:1.1?}\x1b[0m",
            elapsed
        );
    }
    println!("{part1}");
    println!();

    store.data_mut().reset();

    println!("\x1b[4m\x1b[36mPart 2\x1b[0m");
    let start = Instant::now();
    let part2 = runner.call_solve_b(&mut store, resource)?;
    let elapsed = start.elapsed();
    if store.data().did_log() {
        println!(
            "\x1b[35m\x1b[2mtook:\x1b[0m\x1b[35m {:1.1?}\x1b[0m",
            elapsed
        );
    } else {
        println!(
            "\x1b[F\x1b[K\x1b[4m\x1b[36mPart 2\x1b[0m \x1b[35m{:1.1?}\x1b[0m",
            elapsed
        );
    }
    println!("{part2}");

    resource.resource_drop(&mut store)?;

    Ok(())
}
