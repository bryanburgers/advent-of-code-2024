extern = $(shell find . -type f -name '*.rs' -o -name '*.toml' -o -name '*.wit')
runner_extern = $(shell find wit runner -type f -name '*.rs' -o -name '*.toml' -o -name '*.wit')

.SECONDARY:

clean: 
	rm -rf target/wasm32-unknown-unknown/release/*.wasm
	rm -rf target/wasm32-unknown-unknown/debug/*.wasm
	rm -rf build/*.wasm

target/wasm32-unknown-unknown/release/%.wasm: $(extern)
	cargo component build --target wasm32-unknown-unknown --package $(*F) --release

build/%_parser.wasm: target/wasm32-unknown-unknown/release/%_parser.wasm
	mkdir -p build
	cp $^ $@

build/%_solver.wasm: target/wasm32-unknown-unknown/release/%_solver.wasm
	mkdir -p build
	cp $^ $@

build/%.wasm: build/%_parser.wasm build/%_solver.wasm
	wac plug build/$*_parser.wasm --plug build/$*_solver.wasm --output $@

target/release/runner: $(runner_extern)
	cargo build --release --bin runner

%-example: target/release/runner build/%.wasm
	target/release/runner build/$*.wasm $*/example.txt

%-final: target/release/runner build/%.wasm
	target/release/runner build/$*.wasm $*/input.txt
