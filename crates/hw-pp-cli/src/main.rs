use crate::constants::cli_info;
use anyhow::Context;
use clap::Parser;
use hw_pp_core::data::human_languages::hello_world_for;

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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.count == 0 {
        let greeting =
            hello_world_for(None, None).context("failed to look up a hello-world greeting")?;
        println!("{greeting}");
        return Ok(());
    }

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    Ok(())
}
