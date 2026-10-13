use crate::constants::cli_info;
use anyhow::Context;
use clap::Parser;
use hw_pp_core::data::human_languages::{
    DialectName, HumanLanguageName, get_human_language_hello_world_for,
};

mod constants;

#[derive(Parser, Debug)]
#[command(
    name = cli_info::APP_NAME,
    version,
    about = cli_info::about(),
    long_about = cli_info::long_about()
)]
struct Args {
    /// Human language to greet in (e.g. "English", "Ancient Greek")
    #[arg(short = 'l', long, visible_alias = "hl")]
    human_language: Option<HumanLanguageName>,

    #[arg(short = 'd', long, visible_alias = "d")]
    dialect: Option<DialectName>,
}

fn main() -> anyhow::Result<()> {
    // argv[0] is the program name, so a length of 1 means no args were passed.
    if std::env::args_os().len() == 1 {
        let hello_world = get_human_language_hello_world_for(None, None, None)
            .context("failed to look up a 'hello-world'")?;
        println!("{hello_world}");
        return Ok(());
    }

    let args = Args::parse();
    let hello_world = get_human_language_hello_world_for(args.human_language, args.dialect, None)
        .context("failed to look up a 'hello-world'")?;
    println!("{hello_world}");

    Ok(())
}
