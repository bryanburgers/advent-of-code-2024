wasmtime::component::bindgen!("solver" in "solver/wit");

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let filename = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("filename required"))?;
    let input = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("input required"))?;
    let bytes = std::fs::read(filename)?;
    let input = std::fs::read_to_string(input)?;

    let engine = wasmtime::Engine::default();
    let mut store = wasmtime::Store::new(&engine, ());
    let linker = wasmtime::component::Linker::new(&engine);
    let component = wasmtime::component::Component::new(&engine, &bytes)?;
    let instance = Solver::instantiate(&mut store, &component, &linker)?;

    let input = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let a = iter.next().unwrap().parse().unwrap();
            let b = iter.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    let result = instance.call_solve_a(&mut store, &input)?;
    println!("{}", result);
    let result = instance.call_solve_b(&mut store, &input)?;
    println!("{}", result);

    Ok(())
}
