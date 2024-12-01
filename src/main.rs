mod day1;

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

    // tmpl:fn_call
}
