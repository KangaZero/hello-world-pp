use crate::constants::cli_info::{self, BANNER};
use crate::tools::helpers::{
    hello_world_output_with_options, output_list_command_with, snippet_output_with_options,
};
use anstream::{eprintln, println};
use anyhow::Context;
use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use hw_pp_core::data::common::{
    get_default_human_language_hello_world, replace_default_hello_world_string_with,
};
use hw_pp_core::data::human_languages::{
    HumanLanguageCategory, get_all_human_language_categories_as_str_by,
    get_all_human_language_dialects_as_str_by, get_all_human_languages_as_str_by,
    get_all_scripts_as_str_by,
};
use hw_pp_core::data::notation::Notation;
use hw_pp_core::data::programming_languages::{
    ProgrammingLanguageCategory, ProgrammingLanguageFamily,
    get_all_programming_language_categories_as_str_by,
    get_all_programming_language_families_as_str_by, get_all_programming_languages_as_str_by,
    get_all_programming_versions_as_str_by, get_hello_world_version_for,
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
    #[command(visible_alias = "hl")]
    HumanLanguages {
        #[arg(short = 's', long)]
        script: Option<ScriptName>,

        #[arg(short = 'c', long)]
        category: Option<HumanLanguageCategory>,
    },
    #[command(visible_alias = "hd", alias = "human-dialect")]
    HumanDialects { language: Option<HumanLanguageName> },
    #[command(visible_alias = "s", alias = "script")]
    Scripts { language: Option<HumanLanguageName> },
    #[command(visible_alias = "pl", alias = "programming-language")]
    ProgrammingLanguages {
        #[arg(short, long)]
        extension: Option<FileExtension>,
        #[arg(short, long)]
        family: Option<ProgrammingLanguageFamily>,
        #[arg(short, long)]
        category: Option<ProgrammingLanguageCategory>,
    },
    /// Programming Language dialects/versions
    #[command(visible_alias = "pd", alias = "programming-dialect")]
    ProgrammingDialects {
        language: Option<ProgrammingLanguageName>,
    },
    #[command(visible_alias = "hc", alias = "human-language-category")]
    HumanLanguageCategories { language: Option<HumanLanguageName> },
    #[command(visible_alias = "pc", alias = "programming-language-category")]
    ProgrammingLanguageCategories {
        language: Option<ProgrammingLanguageName>,
    },
    #[command(visible_alias = "pf", alias = "programming-language-family")]
    ProgrammingLanguageFamilies {
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

        #[arg(visible_alias = "cnt", long, default_value_t = false)]
        count: bool,
    },
}

#[derive(Parser, Debug)]
#[command(
    name = cli_info::APP_NAME,
    version,
    styles = cli_info::STYLES,
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
    #[arg(long, visible_alias = "np", group = STYLING, default_value_t = false )]
    no_punctuation: bool,

    /// Notation of hello-world output
    #[arg(long, visible_alias = "no", group = STYLING, default_value_t = Notation::Utf8 )]
    notation: Notation,

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
    #[arg(short = 't', long, group = STYLING, default_value_t = Themes::Kanga)]
    theme: Themes,

    /// Outputs only the banner/logo
    #[arg(short = 'b', long, visible_alias = "logo", group = LOGO)]
    banner: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

impl Args {
    /// Whether `--human-language`, `--human-dialect` or `--script` was given.
    fn has_human_options(&self) -> bool {
        self.human_language.is_some() || self.human_dialect.is_some() || self.script.is_some()
    }

    /// Whether `--programming-language`, `--programming-dialect` or `--extension` was given.
    fn has_programming_options(&self) -> bool {
        self.programming_language.is_some()
            || self.programming_dialect.is_some()
            || self.extension.is_some()
    }

    //TODO: Add more logic when config toml and ColorWhen::Always & ColorWhen::Never is added
    /// Whether `--color` allows coloured output (`yes`, the default).
    fn has_color_on(&self) -> bool {
        self.color == ColorWhen::Yes
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(Command::List {
        ref category,
        compact,
        count,
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
            ListCategory::ProgrammingLanguages {
                extension,
                family,
                category,
            } => get_all_programming_languages_as_str_by(*extension, *family, *category),
            ListCategory::ProgrammingDialects { language } => {
                get_all_programming_versions_as_str_by(*language)
            }
            ListCategory::HumanLanguageCategories { language } => {
                get_all_human_language_categories_as_str_by(*language)
            }
            ListCategory::ProgrammingLanguageCategories { language } => {
                get_all_programming_language_categories_as_str_by(*language)
            }
            ListCategory::ProgrammingLanguageFamilies { language } => {
                get_all_programming_language_families_as_str_by(*language)
            }
        };

        output_list_command_with(&list_as_str, compact, count);
        return Ok(());
    }
    if args.banner {
        println!("{}", BANNER);
        return Ok(());
    }

    match (args.has_human_options(), args.has_programming_options()) {
        (false, true) => {
            let language_version = get_hello_world_version_for(
                args.programming_language,
                args.programming_dialect,
                args.extension,
            )
            .context("failed to look up a 'hello-world' programming language version")?;
            let snippet = if args.has_color_on() {
                collect_tokenized_syntax(language_version, Some(args.theme))
            } else {
                language_version.syntax.to_owned()
            };
            println!("{}", snippet_output_with_options(&args, snippet));
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
            let language_version = get_hello_world_version_for(
                args.programming_language,
                args.programming_dialect,
                args.extension,
            )
            .context("failed to look up a 'hello-world' programming language version")?;
            let snippet = if args.has_color_on() {
                collect_tokenized_syntax(language_version, Some(args.theme))
            } else {
                language_version.syntax.to_owned()
            };

            let hello_world_in_human_language = get_human_language_hello_world_for(
                args.human_language,
                args.human_dialect,
                args.script,
            )
            .context("failed to look up a 'hello-world' for human language")?;
            // `-n` strips the greeting before it goes in, so the code stays valid.
            let hello_world_options_applied =
                hello_world_output_with_options(&args, hello_world_in_human_language);
            let result =
                replace_default_hello_world_string_with(&snippet, &hello_world_options_applied);
            // An unchanged snippet (for a greeting other than the default) means the message is
            // encoded (e.g. Brainfuck): say why the greeting is missing; the snippet is still valid.
            if result == snippet
                && hello_world_options_applied != get_default_human_language_hello_world()
            {
                eprintln!(
                    "warning: this snippet encodes its message, so {hello_world_in_human_language:?} \
                     could not be inserted"
                );
            }
            println!("{result}");
        }
    }

    Ok(())
}
