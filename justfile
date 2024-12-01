_default:
    just --list

add_day day_number:
    templative template --day_number {{day_number}}

# Makefile targets

benchmark_day day_number:
    make benchmark_day{{day_number}}

benchmark_all_individually:
    make benchmark_all_individually

benchmark_all:
    make benchmark_all
