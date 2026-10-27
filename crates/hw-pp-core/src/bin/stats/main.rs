//! Developer-only catalog stats: counts and gaps in the human and programming
//! language data. Run with `cargo run -p hw-pp-core --bin stats`.

mod human_language_stats;
mod programming_language_stats;

use std::fmt::Debug;

use anstyle::Style;
use human_language_stats::{HUMAN_LANGUAGE_STATS, HumanLanguageStats};
use hw_pp_core::data::human_languages::ScriptName;
use programming_language_stats::{PROGRAMMING_LANGUAGE_STATS, ProgrammingLanguageStats};

const BOLD: Style = Style::new().bold();
const UNDERLINE: Style = Style::new().underline();
const ITALIC: Style = Style::new().italic();

/// Gap lists longer than this print only their count.
const MAX_LISTED: usize = 50;

fn print_heading(title: &str) {
    println!("{BOLD}{UNDERLINE}<{title}>{BOLD:#}{UNDERLINE:#}");
}

fn print_section(title: &str) {
    println!("\n{UNDERLINE}{title}{UNDERLINE:#}");
}

fn print_total(label: &str, count: usize) {
    println!("Total {label} count: {ITALIC}{count}{ITALIC:#}");
}

/// The count of `languages` missing `what`, and their names when there are at most
/// [`MAX_LISTED`].
fn print_missing<T: Debug>(what: &str, languages: &[T]) {
    let count = languages.len();
    if count <= MAX_LISTED {
        println!("Languages missing {what}: {ITALIC}{count}{ITALIC:#} {languages:?}");
    } else {
        println!("Languages missing {what}: {ITALIC}{count}{ITALIC:#} (too many to list)");
    }
}

fn print_human_language_stats() {
    let stats = &*HUMAN_LANGUAGE_STATS;
    print_heading("Human Language Stats");
    print_total("language", stats.language_count);
    print_total("dialect", stats.dialect_count);
    print_total("greeting", stats.greeting_count);
    print_total("script", stats.scripts.len());
    print_total("category", stats.category_count);
    print_total("family", stats.family_count);

    print_section("Missing INFO");
    print_missing(
        &format!("{} script", ScriptName::Latin),
        &HumanLanguageStats::languages_without_script(Some(ScriptName::Latin)),
    );
    print_missing(
        "a script",
        &HumanLanguageStats::languages_without_script(None),
    );
    print_missing(
        "a category",
        &HumanLanguageStats::languages_without_category(),
    );
    print_missing(
        "a family (isolates, constructed, creoles and pidgins excluded)",
        &HumanLanguageStats::languages_without_family(),
    );
    print_missing(
        "a dialect",
        &HumanLanguageStats::languages_without_dialect(),
    );
    print_missing(
        "a greeting",
        &HumanLanguageStats::languages_without_greeting(),
    );
}

fn print_programming_language_stats() {
    let stats = &*PROGRAMMING_LANGUAGE_STATS;
    print_heading("Programming Language Stats");
    print_total("language", stats.language_count);
    print_total("dialect", stats.dialect_count);
    print_total("extension", stats.extension_count);
    print_total("category", stats.category_count);
    print_total("family", stats.family_count);
    print_total("paradigm", stats.paradigm_count);

    print_section("Missing INFO");
    print_missing(
        "a category",
        &ProgrammingLanguageStats::languages_without_category(),
    );
    print_missing(
        "a family",
        &ProgrammingLanguageStats::languages_without_family(),
    );
    print_missing(
        "a paradigm",
        &ProgrammingLanguageStats::languages_without_paradigm(),
    );
    print_missing(
        "an extension",
        &ProgrammingLanguageStats::languages_without_extension(),
    );
    print_missing(
        "syntax",
        &ProgrammingLanguageStats::languages_without_syntax(),
    );
    print_missing(
        "tokenized syntax",
        &ProgrammingLanguageStats::languages_without_tokenized_syntax(),
    );
}

fn main() {
    print_human_language_stats();
    println!("\n");
    print_programming_language_stats();
}
