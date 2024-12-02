mod day1;

mod day2;

// tmpl:mod
mod helpers;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(long)]
    single: bool,
    #[arg(long)]
    day1: bool,

    #[arg(long)]
    day2: bool,

    // tmpl:arg
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    if !args.single || args.day1 {
        let (day1_part1, day1_part2) = day1::day1(None).await;
        println!("Day 1 Part 1: {}", day1_part1);
        println!("Day 1 Part 2: {}", day1_part2);
    }

    if !args.single || args.day2 {
        let (day2_part1, day2_part2) = day2::day2(None).await;
        println!("Day 2 Part 1: {}", day2_part1);
        println!("Day 2 Part 2: {}", day2_part2);
    }

    // tmpl:fn_call
}
