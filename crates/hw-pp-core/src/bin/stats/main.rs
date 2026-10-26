//! Developer-only catalog stats: counts and gaps in the human and programming
//! language data. Run with `cargo run -p hw-pp-core --bin stats`.

mod human_language_stats;
mod programming_language_stats;

use anstyle::Style;
use human_language_stats::{HUMAN_LANGUAGE_STATS, HumanLanguageStats};
use hw_pp_core::data::human_languages::ScriptName;
use programming_language_stats::{PROGRAMMING_LANGUAGE_STATS, ProgrammingLanguageStats};

fn main() {
    const BOLD: Style = Style::new().bold();
    const UNDERLINE: Style = Style::new().underline();
    const ITALIC: Style = Style::new().italic();

    // println!("{:#?}", *HUMAN_LANGUAGE_STATS);
    println!("{BOLD}{UNDERLINE}<Human Language Stats>{BOLD:#}{UNDERLINE:#}");
    println!(
        "Total language count: {ITALIC}{}{ITALIC:#}",
        HUMAN_LANGUAGE_STATS.language_count
    );
    println!(
        "Total language dialect count: {ITALIC}{}{ITALIC:#}",
        HUMAN_LANGUAGE_STATS.dialect_count
    );
    println!(
        "Total language script count: {ITALIC}{}{ITALIC:#}",
        HUMAN_LANGUAGE_STATS.scripts.len()
    );
    println!("\n{UNDERLINE}Missing INFO{UNDERLINE:#}");
    println!(
        "Languages missing {} script: {ITALIC}{}{ITALIC:#} {:?}",
        ScriptName::Latin,
        HumanLanguageStats::language_count_without_script(Some(ScriptName::Latin)),
        HumanLanguageStats::languages_without_script(Some(ScriptName::Latin))
    );
    println!(
        "Languages missing a script: {ITALIC}{}{ITALIC:#} {:?}",
        HumanLanguageStats::language_count_without_script(None),
        HumanLanguageStats::languages_without_script(None)
    );
    println!(
        "Languages missing a category: {ITALIC}{}{ITALIC:#} {:?}",
        HumanLanguageStats::languages_without_category().len(),
        HumanLanguageStats::languages_without_category()
    );
    println!(
        "Languages missing a dialect: {ITALIC}{}{ITALIC:#} {:?}",
        HumanLanguageStats::languages_without_dialect().len(),
        HumanLanguageStats::languages_without_dialect()
    );
    println!(
        "Languages missing a greeting: {ITALIC}{}{ITALIC:#} {:?}",
        HumanLanguageStats::languages_without_greeting().len(),
        HumanLanguageStats::languages_without_greeting()
    );

    println!("\n");
    println!("{BOLD}{UNDERLINE}<Programming Language Stats>{BOLD:#}{UNDERLINE:#}");
    println!(
        "Total language count: {ITALIC}{}{ITALIC:#}",
        PROGRAMMING_LANGUAGE_STATS.language_count
    );
    println!(
        "Total dialect count: {ITALIC}{}{ITALIC:#}",
        PROGRAMMING_LANGUAGE_STATS.dialect_count
    );
    println!(
        "Total extension count: {ITALIC}{}{ITALIC:#}",
        PROGRAMMING_LANGUAGE_STATS.extension_count
    );
    println!(
        "Total category count: {ITALIC}{}{ITALIC:#}",
        PROGRAMMING_LANGUAGE_STATS.category_count
    );
    println!(
        "Total family count: {ITALIC}{}{ITALIC:#}",
        PROGRAMMING_LANGUAGE_STATS.family_count
    );
    println!("\n{UNDERLINE}Missing INFO{UNDERLINE:#}");
    println!(
        "Languages missing family: {ITALIC}{}{ITALIC:#}",
        ProgrammingLanguageStats::languages_without_family().len(),
        // Currently too many languages without an extension
        // ProgrammingLanguageStats::languages_without_extension()
    );
    println!(
        "Languages missing extension: {ITALIC}{}{ITALIC:#}",
        ProgrammingLanguageStats::languages_without_extension().len(),
        // Currently too many languages without an extension
        // ProgrammingLanguageStats::languages_without_extension()
    );
    println!(
        "Languages missing syntax: {ITALIC}{}{ITALIC:#} {:?}",
        ProgrammingLanguageStats::languages_without_syntax().len(),
        ProgrammingLanguageStats::languages_without_syntax(),
    );
    println!(
        "Languages missing tokenized syntax: {ITALIC}{}{ITALIC:#}",
        ProgrammingLanguageStats::languages_without_tokenized_syntax().len(),
        // Currently too many languages without an extension
        // ProgrammingLanguageStats::languages_without_tokenized_syntax(),
    );
}
