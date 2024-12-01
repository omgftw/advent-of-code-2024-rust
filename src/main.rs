// tmpl:mod
mod helpers;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(long)]
    single: bool,
    // tmpl:arg
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    // tmpl:fn_call
}
