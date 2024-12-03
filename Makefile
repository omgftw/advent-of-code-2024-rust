WARMUP=100
BINARY_PATH=./target/release/advent-of-code-2023-rust

build:
	cargo build --release

target/release/advent-of-code-2023-rust:
	cargo build --release

benchmark_day1: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --day1'

benchmark_day2: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --day2'

# tmpl:benchmark :prepend
benchmark_all_individually: build benchmark_day1 # tmpl:benchmark_all_individually :append
benchmark_all_individually: benchmark_day2

benchmark_all: build
	@echo "Benchmarking all days on a single run..."
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH}'

clean:
	rm -rf target
