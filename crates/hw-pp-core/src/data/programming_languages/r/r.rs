//! Hello-world snippet for `ProgrammingLanguageName::R`.
//!
//! Sources:
//! - <https://cran.r-project.org/doc/manuals/r-release/R-intro.html>
//! - <https://stat.ethz.ch/R-manual/R-devel/library/base/html/cat.html>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::R, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: R => "R",
    versions: [R4 => "R 4"],
    extensions: [R => "r"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::R,
        extensions: &[FileExtension::R],
        categories: &[Scripting],
        families: &[R],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::R4,
            syntax: concat!(r#"cat(""#, output_for_hello_world!(), r#"\n")"#),
            tokenized_syntax: &[
                (TokenKind::Function, "cat"),
                (TokenKind::Plain, "("),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), r#"\n""#),
                ),
                (TokenKind::Punctuation, ")"),
            ],
        }],
    };
