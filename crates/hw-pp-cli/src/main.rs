use crate::constants::cli_info;
use clap::Parser;

mod constants;

#[derive(Parser, Debug)]
#[command(
    name = cli_info::APP_NAME,
    version,
    about = cli_info::about(),
    long_about = cli_info::long_about()
)]
struct Args {
    /// TODO:
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
