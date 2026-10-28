use std::borrow::Cow;
use std::io;

use crate::constants::cli_info::{self, BANNER};
use crate::tools::helpers::{
    hello_world_output_with_options, output_list_command_with,
    snippet_output_with_default_hello_world_with_options, warn,
};
use anstream::println;
use anyhow::Context;
use clap::Command as ClapCommand;
use clap::error::ErrorKind;
use clap::{ArgGroup, CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::Generator;
use clap_complete::aot::{Shell, generate};
use hw_pp_core::data::common::replace_default_hello_world_string_with;
use hw_pp_core::data::human_languages::{
    DialectName, Greeting, HumanLanguageCategory, HumanLanguageFamily, HumanLanguageName,
    ScriptName,
};
use hw_pp_core::data::notation::Notation;
use hw_pp_core::data::programming_languages::{
    FileExtension, ProgrammingLanguageCategory, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily, ProgrammingLanguageName, ProgrammingLanguageVersion,
    ProgrammingParadigm,
};
use hw_pp_core::data::theme::Theme;

mod constants;
mod tools;

/// Group ID for the human-language options; combines with [`PROGRAMMING`].
const HUMAN: &str = "human";
/// Group ID for the programming-language options; combines with [`HUMAN`].
const PROGRAMMING: &str = "programming";
/// Group ID for `--banner`; conflicts with [`HUMAN`] and [`PROGRAMMING`].
const LOGO: &str = "logo";
/// Group ID that only groups the styling options in `--help`; imposes no constraint.
const STYLING: &str = "styling";

/// When to syntax-highlight programming snippets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum ColorWhen {
    /// Highlight snippets
    Yes,
    /// Print snippets as plain text
    No,
    // TODO: Create $HOME/.config/hw-pp/config.toml
    // Always,
    // Never,
}

/// What `hw-pp list` can list.
#[derive(Subcommand, Debug)]
enum ListCategory {
    /// Human languages
    #[command(short_flag = 'l', visible_alias = "hl")]
    HumanLanguages {
        /// Only languages written in this script
        #[arg(short = 's', long)]
        script: Option<ScriptName>,

        /// Only languages in this category
        #[arg(short = 'c', long)]
        category: Option<HumanLanguageCategory>,

        /// Only languages in this family or branch (e.g. indo-european, bantu)
        #[arg(short = 'f', long)]
        family: Option<HumanLanguageFamily>,
    },
    /// Human language dialects
    #[command(visible_alias = "hd", alias = "human-dialect")]
    HumanDialects {
        /// Only dialects of this language
        language: Option<HumanLanguageName>,
    },
    /// Writing systems (scripts)
    #[command(visible_alias = "s", alias = "script")]
    Scripts {
        /// Only scripts used by this language
        language: Option<HumanLanguageName>,
    },

    /// Greeting notations (`--notation`)
    #[command(visible_alias = "no", alias = "notation")]
    Notations,

    /// Programming languages
    #[command(short_flag = 'p', visible_alias = "pl", alias = "programming-language")]
    ProgrammingLanguages {
        /// Only languages using this file extension
        #[arg(short, long)]
        extension: Option<FileExtension>,
        /// Only languages in this family
        #[arg(short, long)]
        family: Option<ProgrammingLanguageFamily>,
        /// Only languages in this category
        #[arg(short, long)]
        category: Option<ProgrammingLanguageCategory>,
        /// Only languages supporting this paradigm (e.g. functional, object-oriented)
        #[arg(short, long)]
        paradigm: Option<ProgrammingParadigm>,
    },
    /// Programming language dialects (versions)
    #[command(visible_alias = "pd", alias = "programming-dialect")]
    ProgrammingDialects {
        /// Only dialects of this language
        language: Option<ProgrammingLanguageName>,
    },
    /// Human language categories
    #[command(visible_alias = "hc", alias = "human-language-category")]
    HumanLanguageCategories {
        /// Only categories of this language
        language: Option<HumanLanguageName>,
    },
    /// Human language families and branches
    #[command(visible_alias = "hf", alias = "human-language-family")]
    HumanLanguageFamilies {
        /// Only families of this language
        language: Option<HumanLanguageName>,
    },
    /// Programming language categories
    #[command(visible_alias = "pc", alias = "programming-language-category")]
    ProgrammingLanguageCategories {
        /// Only categories of this language
        language: Option<ProgrammingLanguageName>,
    },
    /// Programming language families
    #[command(visible_alias = "pf", alias = "programming-language-family")]
    ProgrammingLanguageFamilies {
        /// Only families of this language
        language: Option<ProgrammingLanguageName>,
    },
    /// Programming paradigms (e.g. functional, object-oriented)
    #[command(visible_alias = "pp", alias = "programming-paradigm")]
    ProgrammingParadigms {
        /// Only paradigms of this language
        language: Option<ProgrammingLanguageName>,
    },

    /// Syntax-highlighting themes
    #[command(visible_alias = "t")]
    Themes,
}

/// Subcommands of `hw-pp`.
#[derive(Subcommand, Debug)]
enum Command {
    /// List the available values for an option
    List {
        /// What to list
        #[command(subcommand)]
        category: ListCategory,

        /// Print entries space-separated on one line
        #[arg(visible_alias = "co", long, default_value_t = false)]
        compact: bool,

        /// Print the number of entries after the list
        #[arg(visible_alias = "cn", long, default_value_t = false)]
        count: bool,

        /// Print in endonym name next to each entry if any. Wrapped in brackets
        #[arg(visible_alias = "en", long, default_value_t = false)]
        endonym: bool,
    },
    /// Generate Shell completions (bash/zsh/fish/elvish/powershell)
    Completions { shell: Shell },
}

/// Command-line options of `hw-pp`.
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
    /// Human language to greet in (e.g. "English", "Ancient Greek"). Defaults to "English" unless --human-dialect is given
    #[arg(short = 'l', long, visible_alias = "hl", group = HUMAN)]
    human_language: Option<HumanLanguageName>,

    /// Dialect of the human language (e.g. "Kansai Japanese").
    #[arg(long, visible_alias = "hd", group = HUMAN)]
    human_dialect: Option<DialectName>,

    /// Script to write the greeting in (e.g. "Katakana", "Braille"). Defaults to the dialect's primary script
    #[arg(short = 's', long, visible_alias = "sc", group = HUMAN)]
    script: Option<ScriptName>,

    /// Programming language to print a hello-world program in (e.g. "Rust", "C++"). Defaults to "C" unless --programming-dialect or --extension is given
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

    /// Encoding for the greeting (e.g. "Base64", "Upper-Hex"); in snippets only the message is encoded
    #[arg(long, visible_alias = "no", group = STYLING, default_value_t = Notation::Utf8 )]
    notation: Notation,

    /// Whether to syntax-highlight programming snippets
    #[arg(
        short = 'c',
        long,
        visible_alias = "cl",
        group = STYLING,
        value_enum,
        default_value_t = ColorWhen::Yes
    )]
    color: ColorWhen,

    /// Syntax-highlighting theme for snippets (e.g. "Catppuccin Latte"); see `hw-pp list themes`
    #[arg(short = 't', long, group = STYLING, default_value_t = Theme::Kanga)]
    theme: Theme,

    /// Print only the banner
    #[arg(short = 'b', long, visible_alias = "logo", group = LOGO)]
    banner: bool,

    /// Subcommand to run instead of printing a greeting
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

    // TODO: Add more logic when config toml and ColorWhen::Always & ColorWhen::Never is added
    /// Whether `--color` allows colored output (`yes`, the default).
    fn has_color_on(&self) -> bool {
        self.color == ColorWhen::Yes
    }

    fn validate(&self) {
        if self.notation == Notation::NATO &&
        let Some(script) = self.script
                //TODO: Change to NATO_COMPATIBLE_SCRIPT
                && script != ScriptName::Latin
        {
            Self::command()
                .error(
                    ErrorKind::ArgumentConflict,
                    format!(
                        "the argument '--script {script}' cannot be used with '--notation nato' \
                             (NATO picks the language's NATO compatible script itself)"
                    ),
                )
                .exit();
        }
    }

    /// `--script`, or Latin when `--notation nato` and the language has a Latin-script greeting.
    fn greeting_script(&self) -> Option<ScriptName> {
        let wants_latin = self.notation == Notation::NATO
            && ScriptName::contains_script(
                self.human_language,
                self.human_dialect,
                ScriptName::Latin,
            );
        if wants_latin {
            Some(ScriptName::Latin)
        } else {
            self.script
        }
    }
}

/// Hello-world snippet for the selected programming language, syntax-highlighted
/// unless `--color no`.
fn selected_snippet(args: &Args) -> anyhow::Result<String> {
    let language_version = ProgrammingLanguageVersion::find(
        args.programming_language,
        args.programming_dialect,
        args.extension,
    )
    .context("failed to look up a 'hello-world' programming language version")?;
    Ok(if args.has_color_on() {
        language_version.highlighted(Some(args.theme))
    } else {
        language_version.syntax.to_owned()
    })
}

impl ListCategory {
    /// The display names this `list` subcommand prints. Languages and dialects keep catalog
    /// order, so a language's default dialect comes first; attribute lists are alphabetical.
    fn entries(&self, has_endonym_arg: bool) -> Vec<Cow<'static, str>> {
        match self {
            ListCategory::HumanLanguages {
                script,
                category,
                family,
            } => HumanLanguageName::all_by(*script, *category, *family)
                .map(|lang| match (lang.endonym(), has_endonym_arg) {
                    (Some(e), true) => Cow::Owned(format!("{} ({e})", lang.as_str())),
                    _ => Cow::Borrowed(lang.as_str()),
                })
                .collect(),
            ListCategory::HumanDialects { language } => DialectName::all_by(*language)
                .map(|dialect| match (dialect.endonym(), has_endonym_arg) {
                    (Some(e), true) => Cow::Owned(format!("{} ({e})", dialect.as_str())),
                    _ => Cow::Borrowed(dialect.as_str()),
                })
                .collect(),
            ListCategory::Scripts { language } => {
                alphabetical(ScriptName::all_by(*language).map(|s| {
                    match (s.endonym(), has_endonym_arg) {
                        (Some(e), true) => Cow::Owned(format!("{} ({e})", s.as_str())),
                        _ => Cow::Borrowed(s.as_str()),
                    }
                }))
            }
            ListCategory::Notations => Notation::ALL
                .iter()
                .map(|n| Cow::Borrowed(n.as_str()))
                .collect(),
            // TODO: Able to filter by dark/light in the future
            ListCategory::Themes => Theme::ALL
                .iter()
                .map(|n| Cow::Borrowed(n.as_str()))
                .collect(),
            ListCategory::ProgrammingLanguages {
                extension,
                family,
                category,
                paradigm,
            } => ProgrammingLanguageName::all_by(*extension, *family, *category, *paradigm)
                .map(|lang| Cow::Borrowed(lang.as_str()))
                .collect(),
            ListCategory::ProgrammingDialects { language } => {
                ProgrammingLanguageDialect::all_by(*language)
                    .map(|d| Cow::Borrowed(d.as_str()))
                    .collect()
            }
            ListCategory::HumanLanguageCategories { language } => alphabetical(
                HumanLanguageCategory::all_by(*language).map(|lang| Cow::Borrowed(lang.as_str())),
            ),
            ListCategory::HumanLanguageFamilies { language } => alphabetical(
                HumanLanguageFamily::all_by(*language).map(|f| Cow::Borrowed(f.as_str())),
            ),
            ListCategory::ProgrammingLanguageCategories { language } => alphabetical(
                ProgrammingLanguageCategory::all_by(*language).map(|c| Cow::Borrowed(c.as_str())),
            ),
            ListCategory::ProgrammingLanguageFamilies { language } => alphabetical(
                ProgrammingLanguageFamily::all_by(*language).map(|f| Cow::Borrowed(f.as_str())),
            ),
            ListCategory::ProgrammingParadigms { language } => alphabetical(
                ProgrammingParadigm::all_by(*language).map(|b| Cow::Borrowed(b.as_str())),
            ),
        }
    }
}

//TODO: Move these fns as well helpers to their own crate hw-pp-tools
/// `names` sorted for display.
fn alphabetical<T: Ord>(names: impl Iterator<Item = T>) -> Vec<T> {
    let mut names: Vec<_> = names.collect();
    names.sort_unstable();
    names
}

fn print_completions<G: Generator>(generator: G, cmd: &mut ClapCommand) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(Command::Completions { shell }) = args.command {
        let mut cmd = Args::command();
        print_completions(shell, &mut cmd);
        return Ok(());
    }

    args.validate();

    if let Some(Command::List {
        ref category,
        endonym,
        compact,
        count,
    }) = args.command
    {
        let list_as_str = category.entries(endonym);

        output_list_command_with(&list_as_str, compact, count, endonym);
        return Ok(());
    }
    if args.banner {
        println!("{}", BANNER);
        return Ok(());
    }
    if args.notation == Notation::NATO
        && args.has_human_options()
        && args.greeting_script() != Some(ScriptName::Latin)
    {
        warn(
            &args,
            "this language has no Latin-script greeting, so NATO keeps its letters as they are\n",
        );
    }

    match (args.has_human_options(), args.has_programming_options()) {
        (false, true) => {
            let snippet = selected_snippet(&args)?;
            println!(
                "{}",
                snippet_output_with_default_hello_world_with_options(&args, &snippet)
            );
        }
        // Human options only, or none at all (the default greeting).
        (_, false) => {
            let hello_world = Greeting::text_for(
                args.human_language,
                args.human_dialect,
                args.greeting_script(),
            )
            .context("failed to look up a 'hello-world' for human language")?;
            println!("{}", hello_world_output_with_options(&args, hello_world));
        }
        (true, true) => {
            let snippet = selected_snippet(&args)?;

            let hello_world_in_human_language = Greeting::text_for(
                args.human_language,
                args.human_dialect,
                args.greeting_script(),
            )
            .context("failed to look up a 'hello-world' for human language")?;
            // `--no-punctuation` and `--notation` are applied to the greeting before it goes
            // in, so the surrounding code stays valid.
            let hello_world_options_applied =
                hello_world_output_with_options(&args, hello_world_in_human_language);
            let result =
                replace_default_hello_world_string_with(&snippet, &hello_world_options_applied);
            // The snippet is unchanged even though the processed greeting differs from the
            // default one, so the message must be encoded (e.g. Brainfuck): say why the greeting
            // is missing; the snippet is still valid.
            if result == snippet && hello_world_options_applied != Greeting::default_text() {
                warn(
                    &args,
                    &format!(
                        "this snippet encodes its message, so {hello_world_in_human_language:?} \
                         could not be inserted"
                    ),
                );
            }
            println!("{result}");
        }
    }

    Ok(())
}
