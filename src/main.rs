mod day1;
mod day2;
mod day3;
mod day4;
// tmpl:mod :prepend :no_newline

mod helpers;

use std::collections::HashMap;

use clap::Parser;
use serde::Serialize;

#[derive(Debug, Parser, Serialize)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(long)]
    day1: bool,
    #[arg(long)]
    day2: bool,
    #[arg(long)]
    day3: bool,
    #[arg(long)]
    day4: bool,
    // tmpl:arg :prepend :no_newline
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    // By default, if no days are specified, we run all days. Check for this dynamically.
    let json = serde_json::to_value(&args).unwrap();
    let hashmap: HashMap<String, bool> = serde_json::from_value::<HashMap<String, serde_json::Value>>(json)
        .unwrap()
        .iter()
        .filter(|(k, _)| k.starts_with("day") && k[3..].parse::<u32>().is_ok())
        .map(|(k, v)| (k.to_string(), v.as_bool().unwrap()))
        .collect();

    let run_all = hashmap.values().all(|v| !v);

    let mut handles = vec![];

    if run_all || args.day1 {
        handles.push(tokio::spawn(async {
            let result = day1::day1(None).await;
            (1, result)
        }));
    }

    if run_all || args.day2 {
        handles.push(tokio::spawn(async {
            let result = day2::day2(None).await;
            (2, result)
        }));
    }

    if run_all || args.day3 {
        handles.push(tokio::spawn(async {
            let result = day3::day3(None).await;
            (3, result)
        }));
    }

    if run_all || args.day4 {
        handles.push(tokio::spawn(async {
            let result = day4::day4(None).await;
            (4, result)
        }));
    }

    // tmpl:fn_call :prepend
    for handle in handles {
        let (day, (part1, part2)) = handle.await.unwrap();
        println!("Day {} Part 1: {}", day, part1);
        println!("Day {} Part 2: {}", day, part2);
    }

}
