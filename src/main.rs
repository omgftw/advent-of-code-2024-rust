mod day1;
mod day2;
mod day3;
// tmpl:mod :prepend :no_newline

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
    #[arg(long)]
    day3: bool,
    // tmpl:arg :prepend :no_newline
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let mut handles = vec![];

    if !args.single || args.day1 {
        handles.push(tokio::spawn(async {
            let result = day1::day1(None).await;
            (1, result)
        }));
    }

    if !args.single || args.day2 {
        handles.push(tokio::spawn(async {
            let result = day2::day2(None).await;
            (2, result)
        }));
    }

    if !args.single || args.day3 {
        handles.push(tokio::spawn(async {
            let result = day3::day3(None).await;
            (3, result)
        }));
    }

    // tmpl:fn_call :prepend
    for handle in handles {
        let (day, (part1, part2)) = handle.await.unwrap();
        println!("Day {} Part 1: {}", day, part1);
        println!("Day {} Part 2: {}", day, part2);
    }

}
