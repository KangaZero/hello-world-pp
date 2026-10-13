use crate::constants::cli_info::{self, BANNER};
use crate::tools::helpers::hello_world_output_with_options;
use crate::tools::highlight::render_highlighted;
use anstream::println;
use anyhow::Context;
use clap::{ArgGroup, Parser};
use hw_pp_core::data::{
    human_languages::{
        DialectName, HumanLanguageName, ScriptName, get_human_language_hello_world_for,
    },
    programming_languages::{
        FileExtension, ProgrammingLanguageDialect, ProgrammingLanguageName,
        get_hello_world_version_for,
    },
};

mod constants;
mod tools;

/// Group IDs for the two mutually exclusive sets of options.
const HUMAN: &str = "human";
const PROGRAMMING: &str = "programming";
const LOGO: &str = "logo";

#[derive(Parser, Debug)]
#[command(
    name = cli_info::APP_NAME,
    version,
    about = cli_info::about(),
    long_about = cli_info::long_about(),
    group(ArgGroup::new(HUMAN).multiple(true)),
    group(ArgGroup::new(PROGRAMMING).multiple(true).conflicts_with(HUMAN)),
    // May be multiple set to true in the future if format opts are added
    group(ArgGroup::new(LOGO).multiple(false).conflicts_with_all([HUMAN, PROGRAMMING])),
)]
pub struct Args {
    /// Human language to greet in (e.g. "English", "Ancient Greek")
    #[arg(short = 'l', long, visible_alias = "hl", group = HUMAN)]
    human_language: Option<HumanLanguageName>,

    /// Dialect of the human language (e.g. "Kansai Japanese")
    #[arg(long, visible_alias = "hd", group = HUMAN)]
    human_dialect: Option<DialectName>,

    /// Script to write the greeting in (e.g. "Katakana", "Braille")
    #[arg(short = 's', long, visible_alias = "scr", group = HUMAN)]
    script: Option<ScriptName>,

    /// Programming language to print a hello-world program in (e.g. "Rust", "C++")
    #[arg(short = 'p', long, visible_alias = "pl", group = PROGRAMMING)]
    programming_language: Option<ProgrammingLanguageName>,

    /// Version of the programming language (e.g. "Python 2", "C++20")
    #[arg(long, visible_alias = "pd", group = PROGRAMMING)]
    programming_dialect: Option<ProgrammingLanguageDialect>,

    /// Source file extension, used to pick the programming language (e.g. "rs")
    #[arg(short = 'e', long, visible_alias = "ext", group = PROGRAMMING)]
    extension: Option<FileExtension>,

    /// Strip punctuation from the output
    #[arg(short = 'n', long, visible_alias = "np")]
    no_punctuation: bool,

    /// Outputs only the banner/logo
    #[arg(short = 'b', long, visible_alias = "logo", group = LOGO)]
    banner: bool,
}

impl Args {
    fn has_human_options(&self) -> bool {
        self.human_language.is_some() || self.human_dialect.is_some() || self.script.is_some()
    }

    fn has_programming_options(&self) -> bool {
        self.programming_language.is_some()
            || self.programming_dialect.is_some()
            || self.extension.is_some()
    }
}

fn main() -> anyhow::Result<()> {
    // clap has already rejected human + programming options together (exit code 2).
    let args = Args::parse();

    if args.banner {
        println!("{}", BANNER);
        return Ok(());
    }

    match (args.has_human_options(), args.has_programming_options()) {
        (false, true) => {
            let hello_world_version = get_hello_world_version_for(
                args.programming_language,
                args.programming_dialect,
                args.extension,
            )
            .context("failed to look up a 'hello-world' program")?;
            // `--no-punctuation` only touches string literals, so the code stays valid.
            println!(
                "{}",
                render_highlighted(hello_world_version, args.no_punctuation)
            );
        }
        // Human options only, or none at all (the default greeting).
        (_, false) => {
            let hello_world = get_human_language_hello_world_for(
                args.human_language,
                args.human_dialect,
                args.script,
            )
            .context("failed to look up a 'hello-world'")?;
            println!("{}", hello_world_output_with_options(&args, hello_world));
        }
        (true, true) => unreachable!("clap's `{PROGRAMMING}` group conflicts with `{HUMAN}`"),
    }

    Ok(())
}
