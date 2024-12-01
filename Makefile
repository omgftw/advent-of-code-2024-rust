WARMUP=5
BINARY_PATH=./target/release/advent-of-code-2023-rust

build:
	cargo build --release

target/release/advent-of-code-2023-rust:
	cargo build --release

benchmark_day1: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day1'

benchmark_day2: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day2'

benchmark_day3: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day3'

benchmark_day4: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day4'

benchmark_day5: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day5'

benchmark_day6: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day6'

benchmark_day7: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day7'

benchmark_day8: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day8'

benchmark_day9: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day9'

benchmark_day10: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day10'

benchmark_day11: build
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH} --single --day11'

benchmark_all_individually: build benchmark_day1 benchmark_day2 benchmark_day3 benchmark_day4 benchmark_day5 benchmark_day6
benchmark_all_individually: benchmark_day7 benchmark_day8 benchmark_day9 benchmark_day10 benchmark_day11
	@echo "Benchmarking all days on a single run..."
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH}'

benchmark_all: build
	@echo "Benchmarking all days on a single run..."
	hyperfine --warmup ${WARMUP} -- '${BINARY_PATH}'

clean:
	rm -rf target
