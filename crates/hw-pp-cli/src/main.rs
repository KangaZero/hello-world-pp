use crate::constants::cli_info::{self, BANNER};
use crate::tools::helpers::{hello_world_output_with_options, output_as_compact_list_or_not};
use anstream::println;
use anyhow::Context;
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use hw_pp_core::data::common::replace_default_hello_world_string_with;
use hw_pp_core::data::human_languages::{
    HumanLanguageCategory, get_all_human_language_categories_as_str_by,
    get_all_human_language_dialects_as_str_by, get_all_human_languages_as_str_by,
    get_all_scripts_as_str_by,
};
use hw_pp_core::data::programming_languages::{
    get_all_programming_language_categories_as_str_by, get_all_programming_languages_as_str_by,
    get_all_programming_versions_as_str_by, get_hello_world_syntax_for,
    get_hello_world_version_for,
};
use hw_pp_core::data::programming_languages_token::{
    Themes, collect_tokenized_syntax, get_all_themes_as_str,
};
use hw_pp_core::data::{
    human_languages::{
        DialectName, HumanLanguageName, ScriptName, get_human_language_hello_world_for,
    },
    programming_languages::{FileExtension, ProgrammingLanguageDialect, ProgrammingLanguageName},
};

mod constants;
mod tools;

/// Group IDs for the two mutually exclusive sets of options.
const HUMAN: &str = "human";
const PROGRAMMING: &str = "programming";
const LOGO: &str = "logo";
const STYLING: &str = "styling";

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum ColorWhen {
    Yes,
    No,
    // TODO: Create $HOME/.config/hw-pp/config.toml
    // Always,
    // Never,
}

#[derive(Subcommand, Debug)]
enum ListCategory {
    /// Human languages
    #[command(visible_alias = "hl")]
    HumanLanguages {
        /// e.g. "Latin", "Cyrillic"
        #[arg(short = 's', long)]
        script: Option<ScriptName>,
        /// e.g. "Latin", "Cyrillic"
        #[arg(short = 'c', long)]
        category: Option<HumanLanguageCategory>,
    },
    /// Human language dialects
    #[command(visible_alias = "hd")]
    HumanDialects {
        /// e.g. "Japanese"
        language: Option<HumanLanguageName>,
    },
    #[command(visible_alias = "s")]
    Scripts { language: Option<HumanLanguageName> },
    #[command(visible_alias = "pl")]
    ProgrammingLanguages {
        /// e.g. "m" (Objective-C and MATLAB)
        extension: Option<FileExtension>,
    },
    /// Programming Language dialects/versions
    #[command(visible_alias = "pd")]
    ProgrammingDialects {
        language: Option<ProgrammingLanguageName>,
    },
    #[command(visible_alias = "hc")]
    HumanLanguageCategory {
        /// e.g. "Japanese"
        language: Option<HumanLanguageName>,
    },
    #[command(visible_alias = "pc")]
    ProgrammingLanguageCategory {
        language: Option<ProgrammingLanguageName>,
    },
    /// Syntax-highlighting themes
    #[command(visible_alias = "t")]
    Themes,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List the available values for an option
    List {
        #[command(subcommand)]
        category: ListCategory,

        #[arg(visible_alias = "com", long, default_value_t = false)]
        compact: bool,
    },
}

#[derive(Parser, Debug)]
#[command(
    name = cli_info::APP_NAME,
    version,
    about = cli_info::about(),
    long_about = cli_info::long_about(),
    group(ArgGroup::new(HUMAN).multiple(true)),
    group(ArgGroup::new(PROGRAMMING).multiple(true)),
    group(ArgGroup::new(STYLING).multiple(true)),
    group(ArgGroup::new(LOGO).multiple(false).conflicts_with_all([HUMAN, PROGRAMMING])),
)]
pub struct Args {
    /// Human language to greet in (e.g. "English", "Ancient Greek"). Defaults to "English"
    #[arg(short = 'l', long, visible_alias = "hl", group = HUMAN)]
    human_language: Option<HumanLanguageName>,

    /// Dialect of the human language (e.g. "Kansai Japanese").
    #[arg(long, visible_alias = "hd", group = HUMAN)]
    human_dialect: Option<DialectName>,

    /// Script to write the greeting in (e.g. "Katakana", "Braille"). Defaults to "Latin"
    #[arg(short = 's', long, visible_alias = "scr", group = HUMAN)]
    script: Option<ScriptName>,

    /// Programming language to print a hello-world program in (e.g. "Rust", "C++"). Defaults to "C"
    #[arg(short = 'p', long, visible_alias = "pl", group = PROGRAMMING)]
    programming_language: Option<ProgrammingLanguageName>,

    /// Version of the programming language (e.g. "Python 2", "C++20")
    #[arg(long, visible_alias = "pd", group = PROGRAMMING)]
    programming_dialect: Option<ProgrammingLanguageDialect>,

    /// Source file extension, used to pick the programming language (e.g. "rs")
    #[arg(short = 'e', long, visible_alias = "ext", group = PROGRAMMING)]
    extension: Option<FileExtension>,

    /// Strip punctuation from the output
    #[arg(short = 'n', long, visible_alias = "np", group = STYLING, default_value_t = false )]
    no_punctuation: bool,

    /// Whether to colour the output
    #[arg(
        short = 'c',
        long,
        visible_alias = "clr",
        group = STYLING,
        value_enum,
        default_value_t = ColorWhen::Yes
    )]
    color: ColorWhen,

    /// Colour theme for syntax highlighting (e.g. "Catppuccin Latte")
    #[arg(short = 't', long, group = STYLING, default_value_t = Themes::CatppuccinMocha)]
    theme: Themes,

    /// Outputs only the banner/logo
    #[arg(short = 'b', long, visible_alias = "logo", group = LOGO)]
    banner: bool,

    #[command(subcommand)]
    command: Option<Command>,
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

    //TODO: Add more logic when config toml and ColorWhen::Always & ColorWhen::Never is added
    fn has_color_on(&self) -> bool {
        self.color == ColorWhen::Yes
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(Command::List {
        ref category,
        compact,
    }) = args.command
    {
        let list_as_str: Vec<&'static str> = match category {
            ListCategory::HumanLanguages { script, category } => {
                get_all_human_languages_as_str_by(*script, *category)
            }
            ListCategory::HumanDialects { language } => {
                get_all_human_language_dialects_as_str_by(*language)
            }
            ListCategory::Scripts { language } => get_all_scripts_as_str_by(*language),
            //TODO: Able to filter by dark/light in the future
            ListCategory::Themes => get_all_themes_as_str(),
            ListCategory::ProgrammingLanguages { extension } => {
                get_all_programming_languages_as_str_by(*extension)
            }
            ListCategory::ProgrammingDialects { language } => {
                get_all_programming_versions_as_str_by(*language)
            }
            ListCategory::HumanLanguageCategory { language } => {
                get_all_human_language_categories_as_str_by(*language)
            }
            ListCategory::ProgrammingLanguageCategory { language } => {
                get_all_programming_language_categories_as_str_by(*language)
            }
        };

        output_as_compact_list_or_not(&list_as_str, compact);
        return Ok(());
    }
    if args.banner {
        println!("{}", BANNER);
        return Ok(());
    }

    match (args.has_human_options(), args.has_programming_options()) {
        (false, true) => {
            if args.has_color_on() {
                let language_version = get_hello_world_version_for(
                    args.programming_language,
                    args.programming_dialect,
                    args.extension,
                )
                .context("failed to look up a 'hello-world' programming language version")?;
                let tokenized_hello_world_syntax =
                    collect_tokenized_syntax(language_version, Some(args.theme));
                println!("{tokenized_hello_world_syntax}");
            } else {
                let hello_world = get_hello_world_syntax_for(
                    args.programming_language,
                    args.programming_dialect,
                    args.extension,
                )
                .context("failed to look up a 'hello-world' syntax for programming language")?;
                println!("{hello_world}");
            }
        }
        // Human options only, or none at all (the default greeting).
        (_, false) => {
            let hello_world = get_human_language_hello_world_for(
                args.human_language,
                args.human_dialect,
                args.script,
            )
            .context("failed to look up a 'hello-world' for human language")?;
            println!("{}", hello_world_output_with_options(&args, hello_world));
        }
        (true, true) => {
            let hello_world_syntax = get_hello_world_syntax_for(
                args.programming_language,
                args.programming_dialect,
                args.extension,
            )
            .context("failed to look up a 'hello-world' syntax for programming language")?;
            let hello_world_in_human_language = get_human_language_hello_world_for(
                args.human_language,
                args.human_dialect,
                args.script,
            )
            .context("failed to look up a 'hello-world' for human language")?;
            let result = replace_default_hello_world_string_with(
                hello_world_syntax,
                hello_world_in_human_language,
            )?;
            println!("{}", hello_world_output_with_options(&args, &result));
        }
    }

    Ok(())
}
